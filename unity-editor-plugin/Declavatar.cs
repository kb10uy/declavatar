using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using UnityEngine;
using UnityEditor;
using VRC.SDK3.Avatars.ScriptableObjects;

namespace KusakaFactory.Declavatar
{
    public sealed class Declavatar
    {
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

        public static void GenerateParametersAsset(Avatar avatar, string basePath, string basename)
        {
            // TODO: Check bits usage
            var parameterDefinitions = avatar.Parameters;
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
            AssetDatabase.CreateAsset(parametersAsset, Path.Combine(basePath, $"{basename}.asset"));
        }

        public static void GenerateMenuAsset(Avatar avatar, string basePath, string filename)
        {
            GenerateMenuAsset(avatar.TopMenuGroup, basePath, filename);
        }

        public static VRCExpressionsMenu GenerateMenuAsset(ExMenuItem.ExMenuGroup menuGroup, string basePath, string basename)
        {
            var menuAsset = ScriptableObject.CreateInstance<VRCExpressionsMenu>();
            foreach (var menuItem in menuGroup.Items)
            {
                if (menuAsset.controls.Count >= 8) break;

                var control = new VRCExpressionsMenu.Control();
                if (menuItem is ExMenuItem.ExMenuGroup submenu)
                {
                    control.type = VRCExpressionsMenu.Control.ControlType.SubMenu;
                    control.name = submenu.Name;
                    control.subMenu = GenerateMenuAsset(submenu, basePath, basename);
                }
                else if (menuItem is ExMenuItem.Button button)
                {
                    control.type = VRCExpressionsMenu.Control.ControlType.Button;
                    control.name = button.Name;
                    control.parameter = new VRCExpressionsMenu.Control.Parameter { name = button.Parameter };
                    control.value = button.Value.ConvertToVRCParameterValue();
                }
                else if (menuItem is ExMenuItem.Toggle toggle)
                {
                    control.type = VRCExpressionsMenu.Control.ControlType.Toggle;
                    control.name = toggle.Name;
                    control.parameter = new VRCExpressionsMenu.Control.Parameter { name = toggle.Parameter };
                    control.value = toggle.Value.ConvertToVRCParameterValue();
                }
                else if (menuItem is ExMenuItem.Radial radial)
                {
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
                }
                else if (menuItem is ExMenuItem.TwoAxis twoAxis)
                {
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
                }
                else if (menuItem is ExMenuItem.FourAxis fourAxis)
                {
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
                }
                else
                {
                    continue;
                }
                menuAsset.controls.Add(control);
            }

            AssetDatabase.CreateAsset(menuAsset, Path.Combine(basePath, $"{basename}-{menuGroup.Id}.asset"));
            return menuAsset;
        }
    }
}
