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

        public static void GenerateParametersAsset(Avatar avatar, string basePath, string filename)
        {
            // TODO: Check bits usage
            var parameterDefinitions = avatar.Parameters;
            var parameters = new List<VRCExpressionParameters.Parameter>();
            foreach (var definition in parameterDefinitions.Values)
            {
                if (definition.SyncType.Type == "Local") continue;

                var parameter = new VRCExpressionParameters.Parameter();
                parameter.name = definition.Name;
                parameter.saved = definition.SyncType.Save ?? false;
                Debug.Log($"{definition.ValueType.Type} : {definition.ValueType.Default} ({definition.ValueType.Default.GetType()})");
                switch (definition.ValueType.Type)
                {
                    case "Int":
                        parameter.valueType = VRCExpressionParameters.ValueType.Int;
                        parameter.defaultValue = (float)(long)definition.ValueType.Default;
                        break;
                    case "Float":
                        parameter.valueType = VRCExpressionParameters.ValueType.Float;
                        parameter.defaultValue = (float)(double)definition.ValueType.Default;
                        break;
                    case "Bool":
                        parameter.valueType = VRCExpressionParameters.ValueType.Bool;
                        parameter.defaultValue = (bool)definition.ValueType.Default ? 1.0f : 0.0f;
                        break;
                    default:
                        throw new ArgumentException("invalid parameter type");
                }
                parameters.Add(parameter);
            }

            var parametersAsset = ScriptableObject.CreateInstance<VRCExpressionParameters>();
            parametersAsset.parameters = parameters.ToArray();
            AssetDatabase.CreateAsset(parametersAsset, Path.Combine(basePath, filename));
        }
    }
}
