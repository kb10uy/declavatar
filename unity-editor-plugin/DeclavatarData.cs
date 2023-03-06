using System;
using System.Collections.Generic;
using System.Linq;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace KusakaFactory.Declavatar
{
    public sealed class Avatar
    {
        public string Name { get; set; }
        public Dictionary<string, Parameter> Parameters { get; set; }
        public List<AnimationGroup> AnimationGroups { get; set; }
        public List<DriverGroup> DriverGroups { get; set; }
        public ExMenuItem.ExMenuGroup TopMenuGroup { get; set; }
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
        public string Type { get; set; }
        public bool? Save { get; set; }
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

    [JsonConverter(typeof(Converters.ExMenuItemConverter))]
    public sealed class ExMenuItem
    {
        public sealed class ExMenuGroup
        {
            public string Name { get; set; }
            public uint Id { get; set; }
            public List<object> Items { get; set; }
        }

        public sealed class Button
        {
            public string Name { get; set; }
            public string Parameter { get; set; }
            public ParameterType Value { get; set; }
        }

        public sealed class Toggle
        {
            public string Name { get; set; }
            public string Parameter { get; set; }
            public ParameterType Value { get; set; }
        }

        public sealed class Radial
        {
            public string Name { get; set; }
            public string Parameter { get; set; }
        }

        public sealed class TwoAxis
        {
            public string Name { get; set; }
            public BiAxis HorizontalAxis { get; set; }
            public BiAxis VerticalAxis { get; set; }
        }

        public sealed class FourAxis
        {
            public string Name { get; set; }
            public UniAxis LeftAxis { get; set; }
            public UniAxis RightAxis { get; set; }
            public UniAxis UpAxis { get; set; }
            public UniAxis DownAxis { get; set; }
        }

        public sealed class BiAxis
        {
            public string Parameter { get; set; }
            public string LabelNegative { get; set; }
            public string LabelPositive { get; set; }
        }

        public sealed class UniAxis
        {
            public string Parameter { get; set; }
            public string Label { get; set; }
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

        public sealed class ExMenuItemConverter : JsonConverter
        {
            public override bool CanConvert(Type objectType)
            {
                return objectType == typeof(ExMenuItem);
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
                switch (obj["type"].Value<string>())
                {
                    case "SubMenu": return contentObject.ToObject<ExMenuItem.ExMenuGroup>();
                    case "Button": return contentObject.ToObject<ExMenuItem.Button>();
                    case "Toggle": return contentObject.ToObject<ExMenuItem.Toggle>();
                    case "Radial": return contentObject.ToObject<ExMenuItem.Radial>();
                    case "TwoAxis": return contentObject.ToObject<ExMenuItem.TwoAxis>();
                    case "FourAxis": return contentObject.ToObject<ExMenuItem.FourAxis>();
                    default: throw new JsonException("invalid group type");
                }
            }

            public override void WriteJson(JsonWriter writer, object value, JsonSerializer serializer)
            {
                throw new NotImplementedException();
            }
        }
    }
}
