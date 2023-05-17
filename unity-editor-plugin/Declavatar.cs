using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using UnityEngine;
using UnityEditor;
using UnityEditor.Animations;
using VRC.SDK3.Avatars.Components;
using VRC.SDK3.Avatars.ScriptableObjects;
using AnimatorAsCode.V0;

namespace KusakaFactory.Declavatar
{
    public sealed class Declavatar
    {
        private Avatar _avatar;
        private ExternalAssets _externals;
        private VRCAvatarDescriptor _descriptor;
        private string _basePath;
        private AacFlBase _aac = null;

        public Declavatar(Avatar avatar, ExternalAssets externals, VRCAvatarDescriptor descriptor, string basePath)
        {
            _avatar = avatar;
            _externals = externals;
            _basePath = basePath;
            _descriptor = descriptor;
        }

        public void GenerateTemplateTextAsset()
        {
            var avatarName = _descriptor.gameObject.name;
            var blendShapeMeshes = _descriptor.transform
                .GetComponentsInChildren<SkinnedMeshRenderer>(true)
                .Where((smr) => smr.sharedMesh.blendShapeCount > 0)
                .Select((c) => (
                    Name: c.name,
                    Path: AnimationUtility.CalculateTransformPath(c.transform, _descriptor.transform),
                    Component: c
                ));

            var kdl = new StringBuilder();
            kdl.Append($"version \"1.0.0\"\n");
            kdl.Append($"\n");
            kdl.Append($"avatar \"{avatarName}\" {{\n");
            kdl.Append($"    parameters {{\n");
            foreach (var mesh in blendShapeMeshes) kdl.Append($"        int \"{mesh.Name}\"\n");
            kdl.Append($"    }}\n");
            kdl.Append($"\n");
            kdl.Append($"    animations {{\n");
            foreach (var mesh in blendShapeMeshes)
            {
                kdl.Append($"        group \"{mesh.Name}\" {{\n");
                kdl.Append($"            mesh \"{mesh.Path}\"\n");
                kdl.Append($"            parameter \"{mesh.Name}\"\n");
                kdl.Append($"\n");
                for (var bi = 0; bi < mesh.Component.sharedMesh.blendShapeCount; bi++)
                {
                    kdl.Append($"            option \"{mesh.Component.sharedMesh.GetBlendShapeName(bi)}\"\n");
                }
                kdl.Append($"        }}\n");
            }
            kdl.Append($"    }}\n");
            kdl.Append($"}}\n");

            var kdlBytes = Encoding.UTF8.GetBytes(kdl.ToString());
            var projectPath = Path.GetDirectoryName(Application.dataPath);
            File.WriteAllBytes(Path.Combine(projectPath, _basePath, $"{avatarName}-template.kdl"), kdlBytes);
            AssetDatabase.Refresh();
        }

        public void GenerateAllAssets()
        {
            var containerAnimator = new AnimatorController();
            AssetDatabase.CreateAsset(containerAnimator, Path.Combine(_basePath, $"{_avatar.Name}-container.controller"));
            _aac = AacV0.Create(new AacConfiguration
            {
                DefaultsProvider = new AacDefaultsProvider(),
                SystemName = "Declavatar",
                AssetKey = "Declavatar",
                AvatarDescriptor = _descriptor,
                AnimatorRoot = _descriptor.transform,
                DefaultValueRoot = _descriptor.transform,
                AssetContainer = containerAnimator,
            });

            GenerateParametersAsset();
            GenerateMenuAsset(_avatar.TopMenuGroup);
            GenerateAnimatorsAsset();

            AssetDatabase.Refresh();
        }

        private void GenerateParametersAsset()
        {
            // TODO: Check bits usage
            var parameterDefinitions = _avatar.Parameters;
            var parameters = new List<VRCExpressionParameters.Parameter>();
            foreach (var definition in parameterDefinitions)
            {
                var parameter = definition.ConstructParameter();
                if (parameter != null) parameters.Add(parameter);
            }

            var parametersAsset = ScriptableObject.CreateInstance<VRCExpressionParameters>();
            parametersAsset.parameters = parameters.ToArray();
            AssetDatabase.CreateAsset(parametersAsset, Path.Combine(_basePath, $"{_avatar.Name}-parameters.asset"));
        }

        private VRCExpressionsMenu GenerateMenuAsset(ExMenuItem.ExMenuGroup menuGroup)
        {
            var menuAsset = ScriptableObject.CreateInstance<VRCExpressionsMenu>();
            foreach (var menuItem in menuGroup.Items)
            {
                if (menuAsset.controls.Count >= 8) break;

                var control = new VRCExpressionsMenu.Control();
                switch (menuItem)
                {
                    case ExMenuItem.ExMenuGroup submenu:
                        control.type = VRCExpressionsMenu.Control.ControlType.SubMenu;
                        control.name = submenu.Name;
                        control.subMenu = GenerateMenuAsset(submenu);
                        break;
                    case ExMenuItem.Button button:
                        control.type = VRCExpressionsMenu.Control.ControlType.Button;
                        control.name = button.Name;
                        control.parameter = new VRCExpressionsMenu.Control.Parameter { name = button.Parameter };
                        control.value = button.Value.ConvertToVRCParameterValue();
                        break;
                    case ExMenuItem.Toggle toggle:
                        control.type = VRCExpressionsMenu.Control.ControlType.Toggle;
                        control.name = toggle.Name;
                        control.parameter = new VRCExpressionsMenu.Control.Parameter { name = toggle.Parameter };
                        control.value = toggle.Value.ConvertToVRCParameterValue();
                        break;
                    case ExMenuItem.Radial radial:
                        control.type = VRCExpressionsMenu.Control.ControlType.RadialPuppet;
                        control.name = radial.Name;
                        control.subParameters = new VRCExpressionsMenu.Control.Parameter[]
                        {
                            new VRCExpressionsMenu.Control.Parameter { name = radial.Parameter },
                        };
                        control.labels = new VRCExpressionsMenu.Control.Label[]
                        {
                            new VRCExpressionsMenu.Control.Label { name = "Should Insert Label here" },
                        };
                        break;
                    case ExMenuItem.TwoAxis twoAxis:
                        control.type = VRCExpressionsMenu.Control.ControlType.TwoAxisPuppet;
                        control.name = twoAxis.Name;
                        control.subParameters = new[]
                        {
                            new VRCExpressionsMenu.Control.Parameter { name = twoAxis.HorizontalAxis.Parameter },
                            new VRCExpressionsMenu.Control.Parameter { name = twoAxis.VerticalAxis.Parameter },
                        };
                        control.labels = new[]
                        {
                            new VRCExpressionsMenu.Control.Label { name = twoAxis.VerticalAxis.LabelPositive },
                            new VRCExpressionsMenu.Control.Label { name = twoAxis.HorizontalAxis.LabelPositive },
                            new VRCExpressionsMenu.Control.Label { name = twoAxis.VerticalAxis.LabelNegative },
                            new VRCExpressionsMenu.Control.Label { name = twoAxis.HorizontalAxis.LabelNegative },
                        };
                        break;
                    case ExMenuItem.FourAxis fourAxis:
                        control.type = VRCExpressionsMenu.Control.ControlType.FourAxisPuppet;
                        control.name = fourAxis.Name;
                        control.subParameters = new VRCExpressionsMenu.Control.Parameter[]
                        {
                            new VRCExpressionsMenu.Control.Parameter { name = fourAxis.UpAxis.Parameter },
                            new VRCExpressionsMenu.Control.Parameter { name = fourAxis.RightAxis.Parameter },
                            new VRCExpressionsMenu.Control.Parameter { name = fourAxis.DownAxis.Parameter },
                            new VRCExpressionsMenu.Control.Parameter { name = fourAxis.LeftAxis.Parameter },
                        };
                        control.labels = new VRCExpressionsMenu.Control.Label[]
                        {
                            new VRCExpressionsMenu.Control.Label { name = fourAxis.UpAxis.Label },
                            new VRCExpressionsMenu.Control.Label { name = fourAxis.RightAxis.Label },
                            new VRCExpressionsMenu.Control.Label { name = fourAxis.DownAxis.Label },
                            new VRCExpressionsMenu.Control.Label { name = fourAxis.LeftAxis.Label },
                        };
                        break;
                    default:
                        continue;
                }
                menuAsset.controls.Add(control);
            }

            var filenameId = menuGroup.Id == 0 ? $"root" : menuGroup.Id.ToString();
            AssetDatabase.CreateAsset(menuAsset, Path.Combine(_basePath, $"{_avatar.Name}-menu.{filenameId}.asset"));
            return menuAsset;
        }

        private void GenerateAnimatorsAsset()
        {
            var fxAnimator = new AnimatorController();
            AssetDatabase.CreateAsset(fxAnimator, Path.Combine(_basePath, $"{_avatar.Name}-fx.controller"));

            var fxMain = _aac.CreateMainArbitraryControllerLayer(fxAnimator);
            foreach (var animationGroup in _avatar.AnimationGroups)
            {
                switch (animationGroup.Content)
                {
                    case AnimationGroup.Group g:
                        GenerateShapeGroupLayer(fxAnimator, animationGroup.Name, animationGroup.Parameter, g);
                        break;
                    case AnimationGroup.Switch s:
                        GenerateShapeSwitchLayer(fxAnimator, animationGroup.Name, animationGroup.Parameter, s);
                        break;
                    case AnimationGroup.Puppet p:
                        GeneratePuppetLayer(fxAnimator, animationGroup.Name, animationGroup.Parameter, p);
                        break;
                }
            }
            GeneratePreventionLayers(fxAnimator);
        }

        private void GenerateShapeGroupLayer(AnimatorController controller, string name, string parameter, AnimationGroup.Group g)
        {
            var layer = _aac.CreateSupportingArbitraryControllerLayer(controller, name);
            var layerParameter = layer.IntParameter(parameter);
            var searcher = new GameObjectSearcher(_descriptor.gameObject);

            var idleClip = _aac.NewClip($"sg-{name}-0");
            foreach (var target in g.DefaultTargets)
            {
                switch (target)
                {
                    case Target.Shape shape:
                        var smr = searcher.FindSkinnedMeshRenderer(shape.Mesh);
                        idleClip.BlendShape(smr, shape.Name, shape.Value * 100.0f);
                        break;
                    case Target.Object obj:
                        var go = searcher.FindGameObject(obj.Name);
                        idleClip.Toggling(go, obj.Enabled);
                        break;
                    case Target.Material material:
                        var mr = searcher.FindRenderer(material.Mesh);
                        var targetMaterial = _externals.Materials[material.AssetKey];
                        idleClip.SwappingMaterial(mr, (int)material.Slot, targetMaterial);
                        break;
                }
            }
            var idleState = layer.NewState("Disabled", 0, 0).WithAnimation(idleClip);

            foreach (var option in g.Options)
            {
                var clip = _aac.NewClip($"sg-{name}-{option.Order}");
                foreach (var target in option.Targets)
                {
                    switch (target)
                    {
                        case Target.Shape shape:
                            var smr = searcher.FindSkinnedMeshRenderer(shape.Mesh);
                            clip.BlendShape(smr, shape.Name, shape.Value * 100.0f);
                            break;
                        case Target.Object obj:
                            var go = searcher.FindGameObject(obj.Name);
                            clip.Toggling(go, obj.Enabled);
                            break;
                        case Target.Material material:
                            var mr = searcher.FindRenderer(material.Mesh);
                            var targetMaterial = _externals.Materials[material.AssetKey];
                            clip.SwappingMaterial(mr, (int)material.Slot, targetMaterial);
                            break;
                    }
                }
                var state = layer.NewState($"{option.Order} {option.Name}", (int)option.Order / 8 + 1, (int)option.Order % 8).WithAnimation(clip);
                idleState.TransitionsTo(state).When(layerParameter.IsEqualTo((int)option.Order));
                state.Exits().When(layerParameter.IsNotEqualTo((int)option.Order));
            }
        }

        private void GenerateShapeSwitchLayer(AnimatorController controller, string name, string parameter, AnimationGroup.Switch s)
        {
            var layer = _aac.CreateSupportingArbitraryControllerLayer(controller, name);
            var layerParameter = layer.BoolParameter(parameter);
            var searcher = new GameObjectSearcher(_descriptor.gameObject);

            var disabledClip = _aac.NewClip($"ss-{name}-disabled");
            var enabledClip = _aac.NewClip($"ss-{name}-enabled");
            foreach (var target in s.Disabled)
            {
                switch (target)
                {
                    case Target.Shape shape:
                        var smr = searcher.FindSkinnedMeshRenderer(shape.Mesh);
                        disabledClip.BlendShape(smr, shape.Name, shape.Value * 100.0f);
                        break;
                    case Target.Object obj:
                        var go = searcher.FindGameObject(obj.Name);
                        disabledClip.Toggling(go, obj.Enabled);
                        break;
                    case Target.Material material:
                        var mr = searcher.FindRenderer(material.Mesh);
                        var targetMaterial = _externals.Materials[material.AssetKey];
                        disabledClip.SwappingMaterial(mr, (int)material.Slot, targetMaterial);
                        break;
                }
            }
            foreach (var target in s.Enabled)
            {
                switch (target)
                {
                    case Target.Shape shape:
                        var smr = searcher.FindSkinnedMeshRenderer(shape.Mesh);
                        enabledClip.BlendShape(smr, shape.Name, shape.Value * 100.0f);
                        break;
                    case Target.Object obj:
                        var go = searcher.FindGameObject(obj.Name);
                        enabledClip.Toggling(go, obj.Enabled);
                        break;
                    case Target.Material material:
                        var mr = searcher.FindRenderer(material.Mesh);
                        var targetMaterial = _externals.Materials[material.AssetKey];
                        enabledClip.SwappingMaterial(mr, (int)material.Slot, targetMaterial);
                        break;
                }
            }
            var disabledState = layer.NewState("Disabled").WithAnimation(disabledClip);
            var enabledState = layer.NewState("Enabled").WithAnimation(enabledClip);
            disabledState.TransitionsTo(enabledState).When(layerParameter.IsTrue());
            enabledState.TransitionsTo(disabledState).When(layerParameter.IsFalse());
        }

        private void GeneratePuppetLayer(AnimatorController controller, string name, string parameter, AnimationGroup.Puppet puppet)
        {
            var layer = _aac.CreateSupportingArbitraryControllerLayer(controller, name);
            var layerParameter = layer.FloatParameter(parameter);
            var searcher = new GameObjectSearcher(_descriptor.gameObject);

            var groups = puppet.Keyframes
                .SelectMany((kf) => kf.Targets.Select((t) => (kf.Position, Target: t)))
                .GroupBy((p) => p.Target.AsGroupingKey());

            var clip = _aac.NewClip($"p-{name}").NonLooping();
            clip.Animating((e) =>
            {
                foreach (var group in groups)
                {
                    if (group.Key.StartsWith("s://"))
                    {
                        var points = group.Select((p) => (p.Position, Target: p.Target as Target.Shape)).ToList();
                        var smr = searcher.FindSkinnedMeshRenderer(points[0].Target.Mesh);
                        e.Animates(smr, $"blendShape.{points[0].Target.Name}").WithFrameCountUnit((kfs) =>
                        {
                            foreach (var point in points) kfs.Linear(point.Position * 100.0f, point.Target.Value * 100.0f);
                        });
                    }
                    else if (group.Key.StartsWith("o://"))
                    {
                        var points = group.Select((p) => (p.Position, Target: p.Target as Target.Object)).ToList();
                        var go = searcher.FindGameObject(points[0].Target.Name);
                        e.Animates(go).WithFrameCountUnit((kfs) =>
                        {
                            foreach (var point in points) kfs.Constant(point.Position * 100.0f, point.Target.Enabled ? 1.0f : 0.0f);
                        });
                    }
                    else if (group.Key.StartsWith("m://"))
                    {
                        // Use traditional API for matarial swapping
                        var points = group.Select((p) => (p.Position, Target: p.Target as Target.Material)).ToList();
                        var mr = searcher.FindRenderer(points[0].Target.Mesh);

                        var binding = e.BindingFromComponent(mr, $"m_Materials.Array.data[{points[0].Target.Slot}]");
                        var keyframes = points.Select((p) => new ObjectReferenceKeyframe
                        {
                            time = p.Position * 100.0f,
                            value = _externals.Materials[p.Target.AssetKey],
                        }).ToArray();
                        AnimationUtility.SetObjectReferenceCurve(clip.Clip, binding, keyframes);
                    }
                }
            });

            var state = layer.NewState(name).WithAnimation(clip);
            state.MotionTime(layerParameter);
        }

        private void GeneratePreventionLayers(AnimatorController controller)
        {
            var preventions = _avatar.AnimationGroups.Select((ag) =>
            {
                switch (ag.Content)
                {
                    case AnimationGroup.Group g: return (g.Preventions, ag.Parameter, IsInt: true);
                    case AnimationGroup.Switch s: return (s.Preventions, ag.Parameter, IsInt: false);
                    default: return (new Preventions(), null, false);
                }
            });

            var mouthPreventions = preventions.Where((p) => p.Preventions.Mouth).Select((p) => (p.Parameter, p.IsInt)).ToList();
            var mouthPreventionLayer = _aac.CreateSupportingArbitraryControllerLayer(controller, "MouthPrevention");
            var mouthTrackingState = mouthPreventionLayer.NewState("Tracking").TrackingTracks(AacFlState.TrackingElement.Mouth);
            var mouthAnimationState = mouthPreventionLayer.NewState("Animation").TrackingAnimates(AacFlState.TrackingElement.Mouth);

            if (mouthPreventions.Count > 0)
            {
                var (firstName, firstIsInt) = mouthPreventions[0];
                AacFlTransitionContinuation mouthTrackingConditon;
                AacFlTransitionContinuation mouthAnimationCondition;
                if (firstIsInt)
                {
                    var firstParameter = mouthPreventionLayer.IntParameter(firstName);
                    mouthTrackingConditon = mouthAnimationState.TransitionsTo(mouthTrackingState).When(firstParameter.IsEqualTo(0));
                    mouthAnimationCondition = mouthTrackingState.TransitionsTo(mouthAnimationState).When(firstParameter.IsNotEqualTo(0));
                }
                else
                {
                    var firstParameter = mouthPreventionLayer.BoolParameter(firstName);
                    mouthTrackingConditon = mouthAnimationState.TransitionsTo(mouthTrackingState).When(firstParameter.IsFalse());
                    mouthAnimationCondition = mouthTrackingState.TransitionsTo(mouthAnimationState).When(firstParameter.IsTrue());
                }
                foreach (var (name, isInt) in mouthPreventions.Skip(1))
                {
                    if (isInt)
                    {
                        var parameter = mouthPreventionLayer.IntParameter(name);
                        mouthTrackingConditon.And(parameter.IsEqualTo(0));
                        mouthAnimationCondition.Or().When(parameter.IsNotEqualTo(0));
                    }
                    else
                    {
                        var parameter = mouthPreventionLayer.BoolParameter(name);
                        mouthTrackingConditon.And(parameter.IsFalse());
                        mouthAnimationCondition.Or().When(parameter.IsTrue());
                    }
                }
            }

            var eyelidsPreventions = preventions.Where((p) => p.Preventions.Eyelids).Select((p) => (p.Parameter, p.IsInt)).ToList();
            var eyelidsPreventionLayer = _aac.CreateSupportingArbitraryControllerLayer(controller, "EyelidsPrevention");
            var eyelidsTrackingState = eyelidsPreventionLayer.NewState("Tracking").TrackingTracks(AacFlState.TrackingElement.Eyes);
            var eyelidsAnimationState = eyelidsPreventionLayer.NewState("Animation").TrackingAnimates(AacFlState.TrackingElement.Eyes);

            if (eyelidsPreventions.Count > 0)
            {
                var (firstName, firstIsInt) = eyelidsPreventions[0];
                AacFlTransitionContinuation eyelidsTrackingConditon;
                AacFlTransitionContinuation eyelidsAnimationCondition;
                if (firstIsInt)
                {
                    var firstParameter = eyelidsPreventionLayer.IntParameter(firstName);
                    eyelidsTrackingConditon = eyelidsAnimationState.TransitionsTo(eyelidsTrackingState).When(firstParameter.IsEqualTo(0));
                    eyelidsAnimationCondition = eyelidsTrackingState.TransitionsTo(eyelidsAnimationState).When(firstParameter.IsNotEqualTo(0));
                }
                else
                {
                    var firstParameter = eyelidsPreventionLayer.BoolParameter(firstName);
                    eyelidsTrackingConditon = eyelidsAnimationState.TransitionsTo(eyelidsTrackingState).When(firstParameter.IsFalse());
                    eyelidsAnimationCondition = eyelidsTrackingState.TransitionsTo(eyelidsAnimationState).When(firstParameter.IsTrue());
                }
                foreach (var (name, isInt) in eyelidsPreventions.Skip(1))
                {
                    if (isInt)
                    {
                        var parameter = eyelidsPreventionLayer.IntParameter(name);
                        eyelidsTrackingConditon.And(parameter.IsEqualTo(0));
                        eyelidsAnimationCondition.Or().When(parameter.IsNotEqualTo(0));
                    }
                    else
                    {
                        var parameter = eyelidsPreventionLayer.BoolParameter(name);
                        eyelidsTrackingConditon.And(parameter.IsFalse());
                        eyelidsAnimationCondition.Or().When(parameter.IsTrue());
                    }
                }
            }
        }

        public static Avatar Deserialize(string json)
        {
            return JsonConvert.DeserializeObject<Avatar>(json, new JsonSerializerSettings
            {
                ContractResolver = new DefaultContractResolver
                {
                    NamingStrategy = new SnakeCaseNamingStrategy(),
                }
            });
        }
    }

    public sealed class ExternalAssets
    {
        public IReadOnlyDictionary<string, Material> Materials { get; set; }
        public IReadOnlyDictionary<string, Animation> Animations { get; set; }
    }

    internal sealed class GameObjectSearcher
    {
        private GameObject _root = null;
        private Dictionary<string, Renderer> _renderers = new Dictionary<string, Renderer>();
        private Dictionary<string, SkinnedMeshRenderer> _skinnedMeshRenderers = new Dictionary<string, SkinnedMeshRenderer>();
        private Dictionary<string, GameObject> _objects = new Dictionary<string, GameObject>();
        private HashSet<string> _searchedPaths = new HashSet<string>();

        public GameObjectSearcher(GameObject root)
        {
            _root = root;
        }

        public Renderer FindRenderer(string path)
        {
            var cachedPath = $"mr://{path}";
            if (_searchedPaths.Contains(cachedPath))
            {
                return _renderers.TryGetValue(path, out var mr) ? mr : null;
            }
            else
            {
                var mr = _root.transform.Find(path)?.GetComponent<Renderer>();
                _searchedPaths.Add(cachedPath);
                _renderers[path] = mr;
                return mr;
            }
        }

        public SkinnedMeshRenderer FindSkinnedMeshRenderer(string path)
        {
            var cachedPath = $"smr://{path}";
            if (_searchedPaths.Contains(cachedPath))
            {
                return _skinnedMeshRenderers.TryGetValue(path, out var smr) ? smr : null;
            }
            else
            {
                var smr = _root.transform.Find(path)?.GetComponent<SkinnedMeshRenderer>();
                _searchedPaths.Add(cachedPath);
                _skinnedMeshRenderers[path] = smr;
                return smr;
            }
        }

        public GameObject FindGameObject(string path)
        {
            var cachedPath = $"go://{path}";
            if (_searchedPaths.Contains(cachedPath))
            {
                return _objects.TryGetValue(path, out var smr) ? smr : null;
            }
            else
            {
                var go = _root.transform.Find(path)?.gameObject;
                _searchedPaths.Add(cachedPath);
                _objects[path] = go;
                return go;
            }
        }
    }
}
