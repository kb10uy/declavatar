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
        private VRCAvatarDescriptor _descriptor;
        private string _basePath;
        private AacFlBase _aac = null;

        public Declavatar(Avatar avatar, VRCAvatarDescriptor descriptor, string basePath)
        {
            _avatar = avatar;
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
                kdl.Append($"        shape-group \"{mesh.Name}\" {{\n");
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
                if (definition.SyncType.Type == "Local") continue;

                var parameter = new VRCExpressionParameters.Parameter();
                parameter.name = definition.Name;
                parameter.saved = definition.SyncType.Save ?? false;
                parameter.valueType = definition.ValueType.ConvertToVRCParameterType();
                parameter.defaultValue = definition.ValueType.ConvertToVRCParameterValue();
                parameters.Add(parameter);
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
                    case AnimationGroup.ShapeGroup shapeGroup:
                        GenerateShapeGroupLayer(fxAnimator, animationGroup.Name, animationGroup.Parameter, shapeGroup);
                        break;
                    case AnimationGroup.ShapeSwitch shapeSwitch:
                        GenerateShapeSwitchLayer(fxAnimator, animationGroup.Name, animationGroup.Parameter, shapeSwitch);
                        break;
                    case AnimationGroup.ObjectGroup objectGroup:
                        GenerateObjectGroupLayer(fxAnimator, animationGroup.Name, animationGroup.Parameter, objectGroup);
                        break;
                    case AnimationGroup.ObjectSwitch objectSwitch:
                        GenerateObjectSwitchLayer(fxAnimator, animationGroup.Name, animationGroup.Parameter, objectSwitch);
                        break;
                    case AnimationGroup.Puppet puppet:
                        GeneratePuppetLayer(fxAnimator, animationGroup.Name, animationGroup.Parameter, puppet);
                        break;
                }
            }
            GeneratePreventionLayers(fxAnimator);
        }

        private void GenerateShapeGroupLayer(AnimatorController controller, string name, string parameter, AnimationGroup.ShapeGroup shapeGroup)
        {
            var layer = _aac.CreateSupportingArbitraryControllerLayer(controller, name);
            var layerParameter = layer.IntParameter(parameter);
            var renderer = (SkinnedMeshRenderer)_descriptor.transform.Find(shapeGroup.Mesh).GetComponent<SkinnedMeshRenderer>();

            var idleClip = _aac.NewClip($"sg-{name}-0");
            foreach (var shape in shapeGroup.DefaultTargets) idleClip.BlendShape(renderer, shape.Name, shape.Value * 100.0f);
            var idleState = layer.NewState("Disabled", 0, 0).WithAnimation(idleClip);

            foreach (var option in shapeGroup.Options)
            {
                var clip = _aac.NewClip($"sg-{name}-{option.Order}");
                foreach (var shape in option.Shapes) clip.BlendShape(renderer, shape.Name, shape.Value * 100.0f);
                var state = layer.NewState($"{option.Order} {option.Name}", (int)option.Order / 8 + 1, (int)option.Order % 8).WithAnimation(clip);
                idleState.TransitionsTo(state).When(layerParameter.IsEqualTo((int)option.Order));
                state.Exits().When(layerParameter.IsNotEqualTo((int)option.Order));
            }
        }

        private void GenerateShapeSwitchLayer(AnimatorController controller, string name, string parameter, AnimationGroup.ShapeSwitch shapeSwitch)
        {
            var layer = _aac.CreateSupportingArbitraryControllerLayer(controller, name);
            var layerParameter = layer.BoolParameter(parameter);
            var renderer = (SkinnedMeshRenderer)_descriptor.transform.Find(shapeSwitch.Mesh).GetComponent<SkinnedMeshRenderer>();

            var disabledClip = _aac.NewClip($"ss-{name}-disabled");
            var enabledClip = _aac.NewClip($"ss-{name}-enabled");
            foreach (var shape in shapeSwitch.Disabled) disabledClip.BlendShape(renderer, shape.Name, shape.Value * 100.0f);
            foreach (var shape in shapeSwitch.Enabled) enabledClip.BlendShape(renderer, shape.Name, shape.Value * 100.0f);
            var disabledState = layer.NewState("Disabled").WithAnimation(disabledClip);
            var enabledState = layer.NewState("Enabled").WithAnimation(enabledClip);
            disabledState.TransitionsTo(enabledState).When(layerParameter.IsTrue());
            enabledState.TransitionsTo(disabledState).When(layerParameter.IsFalse());
        }

        private void GenerateObjectGroupLayer(AnimatorController controller, string name, string parameter, AnimationGroup.ObjectGroup objectGroup)
        {
            var layer = _aac.CreateSupportingArbitraryControllerLayer(controller, name);
            var layerParameter = layer.IntParameter(parameter);

            var idleClip = _aac.NewClip($"og-{name}-0");
            foreach (var target in objectGroup.DefaultTargets)
            {
                var targetObject = _descriptor.transform.Find(target.Object)?.gameObject;
                idleClip.Toggling(targetObject, target.Enabled);
            }
            var idleState = layer.NewState("Disabled", 0, 0).WithAnimation(idleClip);

            foreach (var option in objectGroup.Options)
            {
                var clip = _aac.NewClip($"og-{name}-{option.Order}");
                foreach (var target in option.Objects)
                {
                    var targetObject = _descriptor.transform.Find(target.Object)?.gameObject;
                    clip.Toggling(targetObject, target.Enabled);
                }
                var state = layer.NewState($"{option.Order} {option.Name}", (int)option.Order / 8 + 1, (int)option.Order % 8).WithAnimation(clip);
                idleState.TransitionsTo(state).When(layerParameter.IsEqualTo((int)option.Order));
                state.Exits().When(layerParameter.IsNotEqualTo((int)option.Order));
            }
        }

        private void GenerateObjectSwitchLayer(AnimatorController controller, string name, string parameter, AnimationGroup.ObjectSwitch objectSwitch)
        {
            var layer = _aac.CreateSupportingArbitraryControllerLayer(controller, name);
            var layerParameter = layer.BoolParameter(parameter);

            var disabledClip = _aac.NewClip($"os-{name}-disabled");
            var enabledClip = _aac.NewClip($"os-{name}-enabled");
            foreach (var target in objectSwitch.Disabled)
            {
                var targetObject = _descriptor.transform.Find(target.Object)?.gameObject;
                disabledClip.Toggling(targetObject, target.Enabled);
            }
            foreach (var target in objectSwitch.Enabled)
            {
                var targetObject = _descriptor.transform.Find(target.Object)?.gameObject;
                enabledClip.Toggling(targetObject, target.Enabled);
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
            var renderer = (SkinnedMeshRenderer)_descriptor.transform.Find(puppet.Mesh).GetComponent<SkinnedMeshRenderer>();

            var groupedByShape = puppet.Keyframes.SelectMany(
                (kf) => kf.Shapes.Select((s) => (
                    Shape: $"blendShape.{s.Name}",
                    Time: (int)(kf.Position * 100.0f),
                    Value: s.Value * 100.0f
                ))
            ).GroupBy((p) => p.Shape);

            var clip = _aac.NewClip($"p-{name}").NonLooping();
            clip.Animating((e) =>
            {
                foreach (var shapeGroup in groupedByShape)
                {
                    e.Animates(renderer, shapeGroup.Key).WithFrameCountUnit((kfs) =>
                    {
                        foreach (var point in shapeGroup) kfs.Linear(point.Time, point.Value);
                    });
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
                    case AnimationGroup.ShapeGroup shapeGroup: return (shapeGroup.PreventMouth, shapeGroup.PreventEyelids, ag.Parameter, IsInt: true);
                    case AnimationGroup.ShapeSwitch shapeSwitch: return (shapeSwitch.PreventMouth, shapeSwitch.PreventEyelids, ag.Parameter, IsInt: false);
                    default: return (false, false, null, false);
                }
            });

            var mouthPreventions = preventions.Where((p) => p.PreventMouth).Select((p) => (p.Parameter, p.IsInt)).ToList();
            var mouthPreventionLayer = _aac.CreateSupportingArbitraryControllerLayer(controller, "MouthPrevention");
            var mouthTrackingState = mouthPreventionLayer.NewState("Tracking").TrackingTracks(AacFlState.TrackingElement.Mouth);
            var mouthAnimationState = mouthPreventionLayer.NewState("Animation").TrackingAnimates(AacFlState.TrackingElement.Mouth);

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

            var eyelidsPreventions = preventions.Where((p) => p.PreventEyelids).Select((p) => (p.Parameter, p.IsInt)).ToList();
            var eyelidsPreventionLayer = _aac.CreateSupportingArbitraryControllerLayer(controller, "EyelidsPrevention");
            var eyelidsTrackingState = eyelidsPreventionLayer.NewState("Tracking").TrackingTracks(AacFlState.TrackingElement.Eyes);
            var eyelidsAnimationState = eyelidsPreventionLayer.NewState("Animation").TrackingAnimates(AacFlState.TrackingElement.Eyes);

            (firstName, firstIsInt) = eyelidsPreventions[0];
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
}
