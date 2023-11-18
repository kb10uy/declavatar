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
        public object Content { get; set; }

        public sealed class Group
        {
            public string Parameter { get; set; }
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
            public string Parameter { get; set; }
            public Preventions Preventions { get; set; }
            public List<Target> Disabled { get; set; }
            public List<Target> Enabled { get; set; }
        }

        public sealed class Puppet
        {
            public string Parameter { get; set; }
            public List<PuppetKeyframe> Keyframes { get; set; }
        }

        public sealed class PuppetKeyframe
        {
            public float Position { get; set; }
            public List<Target> Targets { get; set; }
        }

        public sealed class Layer
        {
            public uint DefaultStateIndex { get; set; }
            public List<LayerState> States { get; set; }
        }

        public sealed class LayerState
        {
            public string Name { get; set; }
            public LayerAnimation Animation { get; set; }
            // public object Speed { get; set; }
            public string Time { get; set; }
            public List<LayerTransition> Transitions { get; set; }
        }

        public sealed class LayerTransition
        {
            public uint Target { get; set; }
            public List<LayerCondition> Conditions { get; set; }
            public float Duration { get; set; }
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

    [JsonConverter(typeof(Converters.LayerAnimationConverter))]
    public abstract class LayerAnimation
    {
        public sealed class Clip : LayerAnimation
        {
            public string AssetKey { get; set; }
        }

        public sealed class BlendTree : LayerAnimation
        {
            public string BlendType { get; set; }
            public List<string> Parameters { get; set; }
            public List<LayerBlendTreeField> Fields { get; set; }
        }
    }

    [JsonConverter(typeof(Converters.LayerBlendTreeFieldConverter))]
    public sealed class LayerBlendTreeField
    {
        public string AssetKey { get; set; }
        public float[] Position { get; set; }
    }

    [JsonConverter(typeof(Converters.LayerConditionConverter))]
    public abstract class LayerCondition
    {
        public sealed class Be : LayerCondition
        {
            public string Parameter { get; set; }
        }

        public sealed class Not : LayerCondition
        {
            public string Parameter { get; set; }
        }

        public sealed class EqInt : LayerCondition
        {
            public string Parameter { get; set; }
            public int Value { get; set; }
        }

        public sealed class NeqInt : LayerCondition
        {
            public string Parameter { get; set; }
            public int Value { get; set; }
        }

        public sealed class GtInt : LayerCondition
        {
            public string Parameter { get; set; }
            public int Value { get; set; }
        }

        public sealed class LeInt : LayerCondition
        {
            public string Parameter { get; set; }
            public int Value { get; set; }
        }

        public sealed class GtFloat : LayerCondition
        {
            public string Parameter { get; set; }
            public float Value { get; set; }
        }

        public sealed class LeFloat : LayerCondition
        {
            public string Parameter { get; set; }
            public float Value { get; set; }
        }
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
                    case "Layer":
                        content = contentObject.ToObject<AnimationGroup.Layer>(serializer);
                        break;
                    default:
                        throw new JsonException("invalid group type");
                }

                return new AnimationGroup
                {
                    Name = obj["name"].Value<string>(),
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

        public sealed class LayerAnimationConverter : JsonConverter
        {
            public override bool CanConvert(Type objectType)
            {
                return objectType == typeof(LayerAnimation);
            }

            public override object ReadJson(JsonReader reader, Type objectType, object existingValue, JsonSerializer serializer)
            {
                var obj = JObject.Load(reader);
                var type = obj["type"].Value<string>();
                var content = obj["content"];
                switch (type)
                {
                    case "Clip": return new LayerAnimation.Clip { AssetKey = content.Value<string>() };
                    case "BlendTree":
                        var ty = content["blend_type"].Value<string>();
                        var parameters = content["params"].Values<string>().ToList();
                        var fields = content["fields"].ToArray().Select((jt) => jt.ToObject<LayerBlendTreeField>()).ToList();
                        return new LayerAnimation.BlendTree { BlendType = ty, Parameters = parameters, Fields = fields };
                    default: throw new JsonException("invalid layer animation type");
                }
            }

            public override void WriteJson(JsonWriter writer, object value, JsonSerializer serializer)
            {
                throw new NotImplementedException();
            }
        }

        public sealed class LayerBlendTreeFieldConverter : JsonConverter
        {
            public override bool CanConvert(Type objectType)
            {
                return objectType == typeof(LayerBlendTreeField);
            }

            public override object ReadJson(JsonReader reader, Type objectType, object existingValue, JsonSerializer serializer)
            {
                var obj = JObject.Load(reader);
                var clip = obj["clip"].Value<string>();
                var position = obj["position"].Values<float>().ToArray();
                return new LayerBlendTreeField { AssetKey = clip, Position = position };
            }

            public override void WriteJson(JsonWriter writer, object value, JsonSerializer serializer)
            {
                throw new NotImplementedException();
            }
        }

        public sealed class LayerConditionConverter : JsonConverter
        {
            public override bool CanConvert(Type objectType)
            {
                return objectType == typeof(LayerCondition);
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
                var content = obj["content"];
                switch (type)
                {
                    case "Be": return new LayerCondition.Be { Parameter = content.Value<string>() };
                    case "Not": return new LayerCondition.Not { Parameter = content.Value<string>() };
                    case "EqInt": return new LayerCondition.EqInt { Parameter = content[0].Value<string>(), Value = content[1].Value<int>() };
                    case "NeqInt": return new LayerCondition.NeqInt { Parameter = content[0].Value<string>(), Value = content[1].Value<int>() };
                    case "GtInt": return new LayerCondition.GtInt { Parameter = content[0].Value<string>(), Value = content[1].Value<int>() };
                    case "LeInt": return new LayerCondition.LeInt { Parameter = content[0].Value<string>(), Value = content[1].Value<int>() };
                    case "GtFloat": return new LayerCondition.GtFloat { Parameter = content[0].Value<string>(), Value = content[1].Value<float>() };
                    case "LeFloat": return new LayerCondition.LeFloat { Parameter = content[0].Value<string>(), Value = content[1].Value<float>() };
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
    }
}
