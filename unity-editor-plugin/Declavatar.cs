using System;
using System.Collections.Generic;
using System.Linq;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Newtonsoft.Json.Serialization;

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
                    NamingStrategy = new SnakeCaseNamingStrategy()
                }
            });
        }

        public sealed class Avatar
        {
            public string Name { get; set; }
            public Dictionary<string, Parameter> Parameters { get; set; }
            public List<AnimationGroup> AnimationGroups { get; set; }
            public List<DriverGroup> DriverGroups { get; set; }
            // public MenuGroup TopMenuGroup { get; set; }
        }

        public sealed class Parameter
        {
            public string Name { get; set; }
            public ParameterType ValueType { get; set; }
            public ParameterSync SyncType { get; set; }
        }

        public sealed class ParameterType
        {
            public string Type { get; set; }
            public object Default { get; set; }
        }

        public sealed class ParameterSync
        {
            public string type { get; set; }
            public bool? save { get; set; }
        }

        [JsonConverter(typeof(Converters.AnimationGroupConverter))]
        public sealed class AnimationGroup
        {
            public string Name { get; set; }
            public string Parameter { get; set; }
            public object Content { get; set; }

            public sealed class ShapeGroup
            {
                public string Mesh { get; set; }
                public bool PreventMouth { get; set; }
                public bool PreventEyelids { get; set; }
                public List<ShapeTarget> DefaultTargets { get; set; }
                public List<ShapeGroupOption> Options { get; set; }
            }

            public sealed class ShapeSwitch
            {
                public string Mesh { get; set; }
                public bool PreventMouth { get; set; }
                public bool PreventEyelids { get; set; }
                public List<ShapeTarget> Disabled { get; set; }
                public List<ShapeTarget> Enabled { get; set; }
            }

            public sealed class ObjectGroup
            {
                public List<ObjectTarget> DefaultTargets { get; set; }
                public List<ObjectGroupOption> Options { get; set; }
            }

            public sealed class ObjectSwitch
            {
                public List<ObjectTarget> Disabled { get; set; }
                public List<ObjectTarget> Enabled { get; set; }
            }

            public sealed class ShapeGroupOption
            {
                public string Name { get; set; }
                public uint Order { get; set; }
                public List<ShapeTarget> Shapes { get; set; }
            }

            public sealed class ObjectGroupOption
            {
                public string Name { get; set; }
                public uint Order { get; set; }
                public List<ObjectTarget> Objects { get; set; }
            }

            public sealed class ShapeTarget
            {
                public string Shape { get; set; }
                public float Value { get; set; }
            }

            public sealed class ObjectTarget
            {
                public string Object { get; set; }
                public bool Enabled { get; set; }
            }
        }

        public sealed class DriverGroup
        {
            public string Name { get; set; }
            public bool Local { get; set; }
            public List<Driver> Drivers { get; set; }
        }

        [JsonConverter(typeof(Converters.DriverConverter))]
        public abstract class Driver
        {
            public sealed class SetInt : Driver
            {
                public string Parameter { get; set; }
                public byte Value { get; set; }
            }

            public sealed class SetFloat : Driver
            {
                public string Parameter { get; set; }
                public float Value { get; set; }
            }

            public sealed class SetBool : Driver
            {
                public string Parameter { get; set; }
                public bool Value { get; set; }
            }

            public sealed class AddInt : Driver
            {
                public string Parameter { get; set; }
                public byte Value { get; set; }
            }

            public sealed class AddFloat : Driver
            {
                public string Parameter { get; set; }
                public float Value { get; set; }
            }

            public sealed class RandomInt : Driver
            {
                public string Parameter { get; set; }
                public byte[] Range { get; set; }
            }

            public sealed class RandomFloat : Driver
            {
                public string Parameter { get; set; }
                public float[] Range { get; set; }
            }

            public sealed class RandomBool : Driver
            {
                public string Parameter { get; set; }
                public float Chance { get; set; }
            }

            public sealed class Copy : Driver
            {
                public string From { get; set; }
                public string To { get; set; }
            }

            public sealed class RangedCopy : Driver
            {
                public string From { get; set; }
                public string To { get; set; }
                public float[] FromRange { get; set; }
                public float[] ToRange { get; set; }
            }
        }

        public static class Converters
        {
            public sealed class AnimationGroupConverter : JsonConverter
            {
                public override bool CanConvert(Type objectType)
                {
                    return objectType == typeof(AnimationGroup);
                }

                public override object ReadJson(
                    JsonReader reader,
                    Type objectType,
                    object existingValue,
                    JsonSerializer serializer
                )
                {
                    var obj = JObject.Load(reader) as JToken;

                    var contentObject = obj["content"] as JObject;
                    object content = null;
                    switch (contentObject["type"].Value<string>())
                    {
                        case "ShapeGroup":
                            content = contentObject.ToObject<AnimationGroup.ShapeGroup>();
                            break;
                        case "ShapeSwitch":
                            content = contentObject.ToObject<AnimationGroup.ShapeSwitch>();
                            break;
                        case "ObjectGroup":
                            content = contentObject.ToObject<AnimationGroup.ObjectGroup>();
                            break;
                        case "ObjectSwitch":
                            content = contentObject.ToObject<AnimationGroup.ObjectSwitch>();
                            break;
                        default:
                            throw new JsonException("invalid group type");
                    }

                    return new AnimationGroup
                    {
                        Name = obj["name"].Value<string>(),
                        Parameter = obj["parameter"].Value<string>(),
                        Content = content,
                    };
                }

                public override void WriteJson(JsonWriter writer, object value, JsonSerializer serializer)
                {
                    throw new NotImplementedException();
                }
            }

            public sealed class DriverConverter : JsonConverter
            {
                public override bool CanConvert(Type objectType)
                {
                    return objectType == typeof(Driver);
                }

                public override object ReadJson(
                    JsonReader reader,
                    Type objectType,
                    object existingValue,
                    JsonSerializer serializer
                )
                {
                    var obj = JObject.Load(reader);
                    var type = obj["type"].Value<string>();
                    var content = obj["content"].Value<JArray>();
                    switch (type)
                    {
                        case "SetInt": return new Driver.SetInt { Parameter = content[0].Value<string>(), Value = content[1].Value<byte>() };
                        case "SetFloat": return new Driver.SetFloat { Parameter = content[0].Value<string>(), Value = content[1].Value<float>() };
                        case "SetBool": return new Driver.SetBool { Parameter = content[0].Value<string>(), Value = content[1].Value<bool>() };
                        case "AddInt": return new Driver.AddInt { Parameter = content[0].Value<string>(), Value = content[1].Value<byte>() };
                        case "AddFloat": return new Driver.AddFloat { Parameter = content[0].Value<string>(), Value = content[1].Value<float>() };
                        case "RandomInt": return new Driver.RandomInt { Parameter = content[0].Value<string>(), Range = content[1].Values<byte>().ToArray() };
                        case "RandomFloat": return new Driver.RandomFloat { Parameter = content[0].Value<string>(), Range = content[1].Values<float>().ToArray() };
                        case "RandomBool": return new Driver.RandomBool { Parameter = content[0].Value<string>(), Chance = content[1].Value<float>() };
                        case "Copy": return new Driver.Copy { From = content[0].Value<string>(), To = content[1].Value<string>() };
                        case "RangedCopy": return new Driver.RangedCopy { From = content[0].Value<string>(), To = content[1].Value<string>(), FromRange = content[2].Values<float>().ToArray(), ToRange = content[3].Values<float>().ToArray() };
                        default: throw new JsonException("invalid driver type");
                    }
                }

                public override void WriteJson(JsonWriter writer, object value, JsonSerializer serializer)
                {
                    throw new NotImplementedException();
                }
            }
        }
    }
}
