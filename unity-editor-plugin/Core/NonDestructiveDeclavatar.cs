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
using AnimatorAsCode.V1;
using AnimatorAsCode.V1.ModularAvatar;
using AnimatorAsCode.V1.VRC;

namespace KusakaFactory.Declavatar
{
    public sealed class NonDestructiveDeclavatar
    {
        private GameObject _rootGameObject;
        private Avatar _declavatarDefinition;
        private IReadOnlyList<ExternalAsset> _externalAssets;
        private AacFlBase _ndmfAac;
        private MaAc _maAc;

        private GameObjectSearcher _searcher;
        private Dictionary<string, (object, Parameter)> _cachedParameters;

        public NonDestructiveDeclavatar(GameObject root, AacFlBase aac, Avatar definition, IReadOnlyList<ExternalAsset> assets)
        {
            _rootGameObject = root;
            _declavatarDefinition = definition;
            _externalAssets = assets;
            _ndmfAac = aac;
            _maAc = MaAc.Create(root);

            _searcher = new GameObjectSearcher(root);
            _cachedParameters = new Dictionary<string, (object, Parameter)>();
        }

        public void Execute()
        {
            GenerateFXLayerNonDestructive();
            GenerateParametersNonDestructive();
            GenerateMenuNonDestructive();
        }

        private void GenerateFXLayerNonDestructive()
        {
            var fxAnimator = _ndmfAac.NewAnimatorController();

            GeneratePreventionLayers(fxAnimator);
            foreach (var animationGroup in _declavatarDefinition.AnimationGroups)
            {
                switch (animationGroup.Content)
                {
                    case AnimationGroup.Group g:
                        GenerateGroupLayer(fxAnimator, animationGroup.Name, g);
                        break;
                    case AnimationGroup.Switch s:
                        GenerateSwitchLayer(fxAnimator, animationGroup.Name, s);
                        break;
                    case AnimationGroup.Puppet p:
                        GeneratePuppetLayer(fxAnimator, animationGroup.Name, p);
                        break;
                    case AnimationGroup.Layer l:
                        GenerateRawLayer(fxAnimator, animationGroup.Name, l);
                        break;
                    default:
                        throw new DeclavatarException("Invalid AnimationGroup deserialization object");
                }
            }

            _maAc.NewMergeAnimator(fxAnimator, VRCAvatarDescriptor.AnimLayerType.FX);
        }

        private void GenerateParametersNonDestructive()
        {
            foreach (var (parameter, definition) in _cachedParameters.Values)
            {
                switch (parameter)
                {
                    case AacFlBoolParameter boolParameter:
                        var boolExParam = _maAc.NewParameter(boolParameter);
                        if (definition.ValueType.Default != null) boolExParam.WithDefaultValue((bool)definition.ValueType.Default);
                        if (definition.Scope.Type != "Synced") boolExParam.NotSynced();
                        if (!(definition.Scope.Save ?? false)) boolExParam.NotSaved();
                        break;
                    case AacFlIntParameter intParameter:
                        var intExParam = _maAc.NewParameter(intParameter);
                        if (definition.ValueType.Default != null) intExParam.WithDefaultValue((int)(long)definition.ValueType.Default);
                        if (definition.Scope.Type != "Synced") intExParam.NotSynced();
                        if (!(definition.Scope.Save ?? false)) intExParam.NotSaved();
                        break;
                    case AacFlFloatParameter floatParameter:
                        var floatExParam = _maAc.NewParameter(floatParameter);
                        if (definition.ValueType.Default != null) floatExParam.WithDefaultValue((float)definition.ValueType.Default);
                        if (definition.Scope.Type != "Synced") floatExParam.NotSynced();
                        if (!(definition.Scope.Save ?? false)) floatExParam.NotSaved();
                        break;
                }
            }
        }

        private void GenerateMenuNonDestructive()
        {
            var rootMenuItem = _maAc.EditMenuItemOnSelf();
        }

        #region FX Layer Generation

        private void GeneratePreventionLayers(AacFlController controller)
        {
            var preventions = _declavatarDefinition.AnimationGroups.Select((ag) =>
            {
                switch (ag.Content)
                {
                    case AnimationGroup.Group g: return (g.Preventions, g.Parameter, IsInt: true);
                    case AnimationGroup.Switch s: return (s.Preventions, s.Parameter, IsInt: false);
                    default: return (new Preventions(), null, false);
                }
            });

            var mouthPreventions = preventions.Where((p) => p.Preventions.Mouth).Select((p) => (p.Parameter, p.IsInt)).ToList();
            var mouthPreventionLayer = controller.NewLayer("MouthPrevention");
            var mouthTrackingState = mouthPreventionLayer.NewState("Tracking").TrackingTracks(AacAv3.Av3TrackingElement.Mouth);
            var mouthAnimationState = mouthPreventionLayer.NewState("Animation").TrackingAnimates(AacAv3.Av3TrackingElement.Mouth);

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
            var eyelidsPreventionLayer = controller.NewLayer("EyelidsPrevention");
            var eyelidsTrackingState = eyelidsPreventionLayer.NewState("Tracking").TrackingTracks(AacAv3.Av3TrackingElement.Eyes);
            var eyelidsAnimationState = eyelidsPreventionLayer.NewState("Animation").TrackingAnimates(AacAv3.Av3TrackingElement.Eyes);

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

        private void GenerateGroupLayer(AacFlController controller, string name, AnimationGroup.Group g)
        {
            var layer = controller.NewLayer(name);
            var layerParameter = CacheIntParameter(g.Parameter, layer);

            var idleClip = _ndmfAac.NewClip($"sg-{name}-0");
            foreach (var target in g.DefaultTargets)
            {
                switch (target)
                {
                    case Target.Shape shape:
                        var smr = _searcher.FindSkinnedMeshRenderer(shape.Mesh);
                        idleClip.BlendShape(smr, shape.Name, shape.Value * 100.0f);
                        break;
                    case Target.Object obj:
                        var go = _searcher.FindGameObject(obj.Name);
                        idleClip.Toggling(go, obj.Enabled);
                        break;
                    case Target.Material material:
                        var mr = _searcher.FindRenderer(material.Mesh);
                        var targetMaterial = SearchExternalMaterial(material.AssetKey);
                        idleClip.SwappingMaterial(mr, (int)material.Slot, targetMaterial);
                        break;
                    default:
                        throw new DeclavatarException("Invalid Target deserialization object");
                }
            }
            var idleState = layer.NewState("Disabled", 0, 0).WithAnimation(idleClip);

            foreach (var option in g.Options)
            {
                var clip = _ndmfAac.NewClip($"sg-{name}-{option.Order}");
                foreach (var target in option.Targets)
                {
                    switch (target)
                    {
                        case Target.Shape shape:
                            var smr = _searcher.FindSkinnedMeshRenderer(shape.Mesh);
                            clip.BlendShape(smr, shape.Name, shape.Value * 100.0f);
                            break;
                        case Target.Object obj:
                            var go = _searcher.FindGameObject(obj.Name);
                            clip.Toggling(go, obj.Enabled);
                            break;
                        case Target.Material material:
                            var mr = _searcher.FindRenderer(material.Mesh);
                            var targetMaterial = SearchExternalMaterial(material.AssetKey);
                            clip.SwappingMaterial(mr, (int)material.Slot, targetMaterial);
                            break;
                        default:
                            throw new DeclavatarException("Invalid Target deserialization object");
                    }
                }
                var state = layer.NewState($"{option.Order} {option.Name}", (int)option.Order / 8 + 1, (int)option.Order % 8).WithAnimation(clip);
                idleState.TransitionsTo(state).When(layerParameter.IsEqualTo((int)option.Order));
                state.Exits().When(layerParameter.IsNotEqualTo((int)option.Order));
            }
        }

        private void GenerateSwitchLayer(AacFlController controller, string name, AnimationGroup.Switch s)
        {
            var layer = controller.NewLayer(name);
            var layerParameter = CacheBoolParameter(s.Parameter, layer);

            var disabledClip = _ndmfAac.NewClip($"ss-{name}-disabled");
            var enabledClip = _ndmfAac.NewClip($"ss-{name}-enabled");
            foreach (var target in s.Disabled)
            {
                switch (target)
                {
                    case Target.Shape shape:
                        var smr = _searcher.FindSkinnedMeshRenderer(shape.Mesh);
                        disabledClip.BlendShape(smr, shape.Name, shape.Value * 100.0f);
                        break;
                    case Target.Object obj:
                        var go = _searcher.FindGameObject(obj.Name);
                        disabledClip.Toggling(go, obj.Enabled);
                        break;
                    case Target.Material material:
                        var mr = _searcher.FindRenderer(material.Mesh);
                        var targetMaterial = SearchExternalMaterial(material.AssetKey);
                        disabledClip.SwappingMaterial(mr, (int)material.Slot, targetMaterial);
                        break;
                    default:
                        throw new DeclavatarException("Invalid Target deserialization object");
                }
            }
            foreach (var target in s.Enabled)
            {
                switch (target)
                {
                    case Target.Shape shape:
                        var smr = _searcher.FindSkinnedMeshRenderer(shape.Mesh);
                        enabledClip.BlendShape(smr, shape.Name, shape.Value * 100.0f);
                        break;
                    case Target.Object obj:
                        var go = _searcher.FindGameObject(obj.Name);
                        enabledClip.Toggling(go, obj.Enabled);
                        break;
                    case Target.Material material:
                        var mr = _searcher.FindRenderer(material.Mesh);
                        var targetMaterial = SearchExternalMaterial(material.AssetKey);
                        enabledClip.SwappingMaterial(mr, (int)material.Slot, targetMaterial);
                        break;
                    default:
                        throw new DeclavatarException("Invalid Target deserialization object");
                }
            }
            var disabledState = layer.NewState("Disabled").WithAnimation(disabledClip);
            var enabledState = layer.NewState("Enabled").WithAnimation(enabledClip);
            disabledState.TransitionsTo(enabledState).When(layerParameter.IsTrue());
            enabledState.TransitionsTo(disabledState).When(layerParameter.IsFalse());
        }

        private void GeneratePuppetLayer(AacFlController controller, string name, AnimationGroup.Puppet puppet)
        {
            var layer = controller.NewLayer(name);
            var layerParameter = CacheFloatParameter(puppet.Parameter, layer);

            var groups = puppet.Keyframes
                .SelectMany((kf) => kf.Targets.Select((t) => (kf.Position, Target: t)))
                .GroupBy((p) => p.Target.AsGroupingKey());

            var clip = _ndmfAac.NewClip($"p-{name}").NonLooping();
            clip.Animating((e) =>
            {
                foreach (var group in groups)
                {
                    if (group.Key.StartsWith("s://"))
                    {
                        var points = group.Select((p) => (p.Position, Target: p.Target as Target.Shape)).ToList();
                        var smr = _searcher.FindSkinnedMeshRenderer(points[0].Target.Mesh);
                        e.Animates(smr, $"blendShape.{points[0].Target.Name}").WithFrameCountUnit((kfs) =>
                        {
                            foreach (var point in points) kfs.Linear(point.Position * 100.0f, point.Target.Value * 100.0f);
                        });
                    }
                    else if (group.Key.StartsWith("o://"))
                    {
                        var points = group.Select((p) => (p.Position, Target: p.Target as Target.Object)).ToList();
                        var go = _searcher.FindGameObject(points[0].Target.Name);
                        e.Animates(go).WithFrameCountUnit((kfs) =>
                        {
                            foreach (var point in points) kfs.Constant(point.Position * 100.0f, point.Target.Enabled ? 1.0f : 0.0f);
                        });
                    }
                    else if (group.Key.StartsWith("m://"))
                    {
                        // Use traditional API for matarial swapping
                        var points = group.Select((p) => (p.Position, Target: p.Target as Target.Material)).ToList();
                        var mr = _searcher.FindRenderer(points[0].Target.Mesh);

                        var binding = e.BindingFromComponent(mr, $"m_Materials.Array.data[{points[0].Target.Slot}]");
                        var keyframes = points.Select((p) => new ObjectReferenceKeyframe
                        {
                            time = p.Position * 100.0f,
                            value = SearchExternalMaterial(p.Target.AssetKey),
                        }).ToArray();
                        AnimationUtility.SetObjectReferenceCurve(clip.Clip, binding, keyframes);
                    }
                }
            });

            var state = layer.NewState(name).WithAnimation(clip);
            state.MotionTime(layerParameter);
        }

        private void GenerateRawLayer(AacFlController controller, string name, AnimationGroup.Layer agLayer)
        {
            var layer = controller.NewLayer(name);

            /*
            // Create states
            var states = new List<AacFlState>();
            foreach (var agState in agLayer.States)
            {
                var state = layer.NewState(agState.Name);
                states.Add(state);
                switch (agState.Animation)
                {
                    case LayerAnimation.Clip clip:
                        state.WithAnimation(_externals.AnimationClips[clip.AssetKey]);
                        if (agState.Time != null)
                        {
                            var speedParameter = layer.FloatParameter(agState.Time);
                            state.MotionTime(speedParameter);
                        }
                        // TODO: Speed parameters
                        break;
                    case LayerAnimation.BlendTree blendTree:
                        var tree = new BlendTree();
                        switch (blendTree.BlendType)
                        {
                            case "Linear":
                                tree.blendType = BlendTreeType.Simple1D;
                                tree.blendParameter = blendTree.Parameters[0];
                                foreach (var field in blendTree.Fields)
                                {
                                    var fieldAnimation = _externals.AnimationClips[field.AssetKey];
                                    tree.AddChild(fieldAnimation, field.Position[0]);
                                }
                                break;
                            case "Simple2D":
                                tree.blendType = BlendTreeType.SimpleDirectional2D;
                                tree.blendParameter = blendTree.Parameters[0];
                                tree.blendParameterY = blendTree.Parameters[1];
                                foreach (var field in blendTree.Fields)
                                {
                                    var fieldAnimation = _externals.AnimationClips[field.AssetKey];
                                    tree.AddChild(fieldAnimation, new Vector2(field.Position[0], field.Position[1]));
                                }
                                break;
                            case "Freeform2D":
                                tree.blendType = BlendTreeType.FreeformDirectional2D;
                                tree.blendParameter = blendTree.Parameters[0];
                                tree.blendParameterY = blendTree.Parameters[1];
                                foreach (var field in blendTree.Fields)
                                {
                                    var fieldAnimation = _externals.AnimationClips[field.AssetKey];
                                    tree.AddChild(fieldAnimation, new Vector2(field.Position[0], field.Position[1]));
                                }
                                break;
                            case "Cartesian2D":
                                tree.blendType = BlendTreeType.FreeformCartesian2D;
                                tree.blendParameter = blendTree.Parameters[0];
                                tree.blendParameterY = blendTree.Parameters[1];
                                foreach (var field in blendTree.Fields)
                                {
                                    var fieldAnimation = _externals.AnimationClips[field.AssetKey];
                                    tree.AddChild(fieldAnimation, new Vector2(field.Position[0], field.Position[1]));
                                }
                                break;
                            default:
                                throw new DeclavatarException($"Invalid BlendTree Type {blendTree.BlendType}");
                        }
                        state.WithAnimation(tree);
                        break;
                }
            }

            // Set transitions
            for (int i = 0; i < states.Count; ++i)
            {
                var fromState = states[i];
                var agState = agLayer.States[i];
                foreach (var transition in agState.Transitions)
                {
                    var targetState = states[(int)transition.Target];
                    var conds = fromState.TransitionsTo(targetState).WithTransitionDurationSeconds(transition.Duration).WhenConditions();
                    foreach (var condBlock in transition.Conditions)
                    {
                        switch (condBlock)
                        {
                            case LayerCondition.Be be:
                                conds.And(layer.BoolParameter(be.Parameter).IsTrue());
                                break;
                            case LayerCondition.Not not:
                                conds.And(layer.BoolParameter(not.Parameter).IsFalse());
                                break;
                            case LayerCondition.EqInt eqInt:
                                conds.And(layer.IntParameter(eqInt.Parameter).IsEqualTo(eqInt.Value));
                                break;
                            case LayerCondition.NeqInt neqInt:
                                conds.And(layer.IntParameter(neqInt.Parameter).IsNotEqualTo(neqInt.Value));
                                break;
                            case LayerCondition.GtInt gtInt:
                                conds.And(layer.IntParameter(gtInt.Parameter).IsGreaterThan(gtInt.Value));
                                break;
                            case LayerCondition.LeInt leInt:
                                conds.And(layer.IntParameter(leInt.Parameter).IsLessThan(leInt.Value));
                                break;
                            case LayerCondition.GtFloat gtFloat:
                                conds.And(layer.FloatParameter(gtFloat.Parameter).IsGreaterThan(gtFloat.Value));
                                break;
                            case LayerCondition.LeFloat leFloat:
                                conds.And(layer.FloatParameter(leFloat.Parameter).IsLessThan(leFloat.Value));
                                break;
                            default:
                                throw new DeclavatarException("Invalid LayerCondition deserialization object");
                        }
                    }
                }
            }
            */
        }

        #endregion

        /*
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
            GeneratePreventionLayers(fxAnimator);
            foreach (var animationGroup in _avatar.AnimationGroups)
            {
                switch (animationGroup.Content)
                {
                    case AnimationGroup.Group g:
                        GenerateShapeGroupLayer(fxAnimator, animationGroup.Name, g);
                        break;
                    case AnimationGroup.Switch s:
                        GenerateShapeSwitchLayer(fxAnimator, animationGroup.Name, s);
                        break;
                    case AnimationGroup.Puppet p:
                        GeneratePuppetLayer(fxAnimator, animationGroup.Name, p);
                        break;
                    case AnimationGroup.Layer l:
                        GenerateRawLayer(fxAnimator, animationGroup.Name, l);
                        break;
                    default:
                        throw new DeclavatarException("Invalid AnimationGroup deserialization object");
                }
            }
        }
        */

        #region External Asset

        private AnimationClip SearchExternalAnimationClip(string key)
        {
            foreach (var assetSet in _externalAssets)
            {
                if (assetSet.Animations == null) continue;
                var value = assetSet.Animations.FirstOrDefault((a) => a.Key == key);
                if (value != null) return value.Animation;
            }
            throw new DeclavatarAssetException($"AnimationClip {key} not defined");
        }

        private Material SearchExternalMaterial(string key)
        {
            foreach (var assetSet in _externalAssets)
            {
                if (assetSet.Materials == null) continue;
                var value = assetSet.Materials.FirstOrDefault((a) => a.Key == key);
                if (value != null) return value.Material;
            }
            throw new DeclavatarAssetException($"Material {key} not defined");
        }

        private string SearchExternalLocalization(string key)
        {
            foreach (var assetSet in _externalAssets)
            {
                if (assetSet.Localizations == null) continue;
                var value = assetSet.Localizations.FirstOrDefault((a) => a.Key == key);
                if (value != null) return value.Localization;
            }
            throw new DeclavatarAssetException($"Localization {key} not defined");
        }

        #endregion

        #region Parameter Caching

        private AacFlBoolParameter CacheBoolParameter(string name, AacFlLayer context)
        {
            var parameterDefinition = _declavatarDefinition.Parameters
                .FirstOrDefault((pd) => pd.Name == name && pd.ValueType.Type == "Bool")
                ?? throw new DeclavatarInternalException($"Parameter '{name}' (bool) not found");

            var aacParameter = context.BoolParameter(name);
            _cachedParameters[name] = (aacParameter, parameterDefinition);
            return aacParameter;
        }

        private AacFlIntParameter CacheIntParameter(string name, AacFlLayer context)
        {
            var parameterDefinition = _declavatarDefinition.Parameters
                .FirstOrDefault((pd) => pd.Name == name && pd.ValueType.Type == "Int")
                ?? throw new DeclavatarInternalException($"Parameter '{name}' (int) not found");

            var aacParameter = context.IntParameter(name);
            _cachedParameters[name] = (aacParameter, parameterDefinition);
            return aacParameter;
        }

        private AacFlFloatParameter CacheFloatParameter(string name, AacFlLayer context)
        {
            var parameterDefinition = _declavatarDefinition.Parameters
                .FirstOrDefault((pd) => pd.Name == name && pd.ValueType.Type == "Float")
                ?? throw new DeclavatarInternalException($"Parameter '{name}' (float) not found");

            var aacParameter = context.FloatParameter(name);
            _cachedParameters[name] = (aacParameter, parameterDefinition);
            return aacParameter;
        }

        #endregion

        #region Object Searching

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
                    var mr = _root.transform.Find(path)?.GetComponent<Renderer>()
                        ?? throw new DeclavatarRuntimeException($"Renderer '{path}' not found");
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
                    var smr = _root.transform.Find(path)?.GetComponent<SkinnedMeshRenderer>()
                        ?? throw new DeclavatarRuntimeException($"SkinnedMeshRenderer '{path}' not found");
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
                    var go = _root.transform.Find(path)?.gameObject
                        ?? throw new DeclavatarRuntimeException($"GameObject '{path}' not found");
                    _searchedPaths.Add(cachedPath);
                    _objects[path] = go;
                    return go;
                }
            }
        }

        #endregion
    }
}
