using System;
using UnityEngine;

namespace KusakaFactory.Declavatar
{
    [CreateAssetMenu(fileName = "DeclavatarExternalAsset", menuName = "Declavatar/External Asset")]
    public class ExternalAsset : ScriptableObject
    {
        public AnimationAsset[] Animations;
        public MaterialAsset[] Materials;
        public LocalizationAsset[] Localizations;
    }

    [Serializable]
    public sealed class AnimationAsset
    {
        public string Key;
        public AnimationClip Animation;
    }

    [Serializable]
    public sealed class MaterialAsset
    {
        public string Key;
        public Material Material;
    }

    [Serializable]
    public sealed class LocalizationAsset
    {
        public string Key;
        public string Localization;
    }
}
