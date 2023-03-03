using System.IO;
using System.Text;
using UnityEngine;
using UnityEditor.Experimental.AssetImporters;

namespace KusakaFactory.Declavatar
{
    [ScriptedImporter(1, "kdl")]
    public sealed class DeclavatarImporter : ScriptedImporter
    {
        public override void OnImportAsset(AssetImportContext ctx)
        {
            var kdlBytes = File.ReadAllBytes(ctx.assetPath);
            var convertedText = Encoding.UTF8.GetString(kdlBytes);
            var textAsset = new TextAsset(convertedText);
            ctx.AddObjectToAsset("MainAsset", textAsset);
            ctx.SetMainObject(textAsset);
        }
    }
}
