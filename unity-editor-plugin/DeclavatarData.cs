using System;
using System.Collections.Generic;
using System.Linq;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using VRC.SDK3.Avatars.ScriptableObjects;

namespace KusakaFactory.Declavatar
{
    public sealed class Avatar
    {
        public string Name { get; set; }
        public List<Parameter> Parameters { get; set; }
        public List<Asset> Assets { get; set; }
        public List<AnimationGroup> AnimationGroups { get; set; }
        public List<DriverGroup> DriverGroups { get; set; }
        public ExMenuItem.ExMenuGroup TopMenuGroup { get; set; }
    }

    public sealed class Parameter
    {
        public string Name { get; set; }
        public ParameterType ValueType { get; set; }
        public ParameterScope Scope { get; set; }
    }

    public sealed class ParameterType
    {
        public string Type { get; set; }
        public object Default { get; set; }
    }

    public sealed class ParameterScope
    {
        public string Type { get; set; }
        public bool? Save { get; set; }
    }

    public sealed class Asset
    {
        public string AssetType { get; set; }
        public string Key { get; set; }
    }

    [JsonConverter(typeof(Converters.AnimationGroupContentConverter))]
    public sealed class AnimationGroup
    {
        public string Name { get; set; }
        public string Parameter { get; set; }
        public object Content { get; set; }

        public sealed class Group
        {
            public Preventions Preventions { get; set; }
            public List<Target> DefaultTargets { get; set; }
            public List<GroupOption> Options { get; set; }
        }

        public sealed class GroupOption
        {
            public string Name { get; set; }
            public uint Order { get; set; }
            public List<Target> Targets { get; set; }
        }

        public sealed class Switch
        {
            public Preventions Preventions { get; set; }
            public List<Target> Disabled { get; set; }
            public List<Target> Enabled { get; set; }
        }

        public sealed class Puppet
        {
            public List<PuppetKeyframe> Keyframes { get; set; }
        }


        public sealed class PuppetKeyframe
        {
            public float Position { get; set; }
            public List<Target> Targets { get; set; }
        }
    }

    [JsonConverter(typeof(Converters.TargetConverter))]
    public abstract class Target
    {
        public sealed class Shape : Target
        {
            public string Mesh { get; set; }
            public string Name { get; set; }
            public float Value { get; set; }
        }

        public sealed class Object : Target
        {
            public string Name { get; set; }
            public bool Enabled { get; set; }
        }

        public sealed class Material : Target
        {
            public string Mesh { get; set; }
            public uint Slot { get; set; }
            public string AssetKey { get; set; }
        }
    }

    public sealed class Preventions
    {
        public bool Mouth { get; set; }
        public bool Eyelids { get; set; }
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

    public abstract class ExMenuItem
    {
        public sealed class ExMenuGroup : ExMenuItem
        {
            public string Name { get; set; }
            public uint Id { get; set; }
            [JsonConverter(typeof(Converters.ExMenuItemsConverter))]
            public List<ExMenuItem> Items { get; set; }
        }

        public sealed class Button : ExMenuItem
        {
            public string Name { get; set; }
            public string Parameter { get; set; }
            public ParameterType Value { get; set; }
        }

        public sealed class Toggle : ExMenuItem
        {
            public string Name { get; set; }
            public string Parameter { get; set; }
            public ParameterType Value { get; set; }
        }

        public sealed class Radial : ExMenuItem
        {
            public string Name { get; set; }
            public string Parameter { get; set; }
        }

        public sealed class TwoAxis : ExMenuItem
        {
            public string Name { get; set; }
            public BiAxis HorizontalAxis { get; set; }
            public BiAxis VerticalAxis { get; set; }
        }

        public sealed class FourAxis : ExMenuItem
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
        public sealed class AnimationGroupContentConverter : JsonConverter
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
                    case "Group":
                        content = contentObject.ToObject<AnimationGroup.Group>(serializer);
                        break;
                    case "Switch":
                        content = contentObject.ToObject<AnimationGroup.Switch>(serializer);
                        break;
                    case "Puppet":
                        content = contentObject.ToObject<AnimationGroup.Puppet>(serializer);
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

        public sealed class TargetConverter : JsonConverter
        {
            public override bool CanConvert(Type objectType)
            {
                return objectType == typeof(Target);
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
                var content = obj["content"] as JObject;
                switch (type)
                {
                    case "Shape": return new Target.Shape { Mesh = content["mesh"].Value<string>(), Name = content["name"].Value<string>(), Value = content["value"].Value<float>(), };
                    case "Object": return new Target.Object { Name = content["name"].Value<string>(), Enabled = content["enabled"].Value<bool>() };
                    case "Material": return new Target.Material { Mesh = content["mesh"].Value<string>(), Slot = content["slot"].Value<uint>(), AssetKey = content["asset_key"].Value<string>() };
                    default: throw new JsonException("invalid driver type");
                }
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

        public sealed class ExMenuItemsConverter : JsonConverter
        {
            public override bool CanConvert(Type objectType)
            {
                return objectType == typeof(List<ExMenuItem>);
            }

            public override object ReadJson(
                JsonReader reader,
                Type objectType,
                object existingValue,
                JsonSerializer serializer
            )
            {
                var items = JArray.Load(reader);
                return items.Select<JToken, ExMenuItem>((item) =>
                {
                    var contentObject = item["content"] as JObject;
                    switch (item["type"].Value<string>())
                    {
                        case "SubMenu": return contentObject.ToObject<ExMenuItem.ExMenuGroup>(serializer);
                        case "Button": return contentObject.ToObject<ExMenuItem.Button>(serializer);
                        case "Toggle": return contentObject.ToObject<ExMenuItem.Toggle>(serializer);
                        case "Radial": return contentObject.ToObject<ExMenuItem.Radial>(serializer);
                        case "TwoAxis": return contentObject.ToObject<ExMenuItem.TwoAxis>(serializer);
                        case "FourAxis": return contentObject.ToObject<ExMenuItem.FourAxis>(serializer);
                        default: throw new JsonException("invalid group type");
                    }
                }).ToList();
            }

            public override void WriteJson(JsonWriter writer, object value, JsonSerializer serializer)
            {
                throw new NotImplementedException();
            }
        }
    }

    public static class VRChatExtension
    {
        public static string AsGroupingKey(this Target target)
        {
            switch (target)
            {
                case Target.Shape s: return $"s://{s.Mesh}/{s.Name}";
                case Target.Object o: return $"o://{o.Name}";
                case Target.Material m: return $"m://{m.Mesh}/{m.Slot}";
                default: throw new ArgumentException("invalid target type");
            }
        }

        public static VRCExpressionParameters.Parameter ConstructParameter(this Parameter definition)
        {
            if (definition.Scope.Type == "Internal") return null;

            var parameter = new VRCExpressionParameters.Parameter();
            parameter.name = definition.Name;
            parameter.saved = definition.Scope.Save ?? false;
            parameter.networkSynced = definition.Scope.Type == "Synced";
            parameter.valueType = definition.ValueType.ConvertToVRCParameterType();
            parameter.defaultValue = definition.ValueType.ConvertToVRCParameterValue();
            return parameter;
        }

        public static VRCExpressionParameters.ValueType ConvertToVRCParameterType(this ParameterType value)
        {
            switch (value.Type)
            {
                case "Int": return VRCExpressionParameters.ValueType.Int;
                case "Float": return VRCExpressionParameters.ValueType.Float;
                case "Bool": return VRCExpressionParameters.ValueType.Bool;
                default: throw new ArgumentException("invalid parameter type");
            }
        }

        public static float ConvertToVRCParameterValue(this ParameterType value)
        {
            switch (value.Type)
            {
                case "Int": return (float)(long)value.Default;
                case "Float": return (float)(double)value.Default;
                case "Bool": return (bool)value.Default ? 1.0f : 0.0f;
                default: throw new ArgumentException("invalid parameter type");
            }
        }
    }
}
