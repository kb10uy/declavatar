using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
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
                }
            }
        }

        private void GenerateShapeGroupLayer(AnimatorController controller, string name, string parameter, AnimationGroup.ShapeGroup shapeGroup)
        {
            var layer = _aac.CreateSupportingArbitraryControllerLayer(controller, name);
            var layerParameter = layer.IntParameter(parameter);
            var renderer = (SkinnedMeshRenderer)_descriptor.transform.Find(shapeGroup.Mesh).GetComponent<SkinnedMeshRenderer>();
        }

        private void GenerateShapeSwitchLayer(AnimatorController controller, string name, string parameter, AnimationGroup.ShapeSwitch shapeSwitch)
        {
            var layer = _aac.CreateSupportingArbitraryControllerLayer(controller, name);
            var layerParameter = layer.BoolParameter(parameter);
        }

        private void GenerateObjectGroupLayer(AnimatorController controller, string name, string parameter, AnimationGroup.ObjectGroup objectGroup)
        {
            var layer = _aac.CreateSupportingArbitraryControllerLayer(controller, name);
            var layerParameter = layer.IntParameter(parameter);
        }

        private void GenerateObjectSwitchLayer(AnimatorController controller, string name, string parameter, AnimationGroup.ObjectSwitch objectSwitch)
        {
            var layer = _aac.CreateSupportingArbitraryControllerLayer(controller, name);
            var layerParameter = layer.BoolParameter(parameter);
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
