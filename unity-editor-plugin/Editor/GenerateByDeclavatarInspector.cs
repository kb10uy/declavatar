using UnityEngine;
using UnityEditor;
using UnityEditorInternal;

namespace KusakaFactory.Declavatar.Editor
{
    [CustomEditor(typeof(GenerateByDeclavatar))]
    public class GenerateByDeclavatarInspector : UnityEditor.Editor
    {
        private SerializedProperty _definitionProperty;
        private SerializedProperty _externalAssetsProperty;

        private ReorderableList _externalAssetsList;


        public void OnEnable()
        {
            _definitionProperty = serializedObject.FindProperty("Definition");
            _externalAssetsProperty = serializedObject.FindProperty("ExternalAssets");

            _externalAssetsList = new ReorderableList(serializedObject, _externalAssetsProperty)
            {
                drawHeaderCallback = (rect) => EditorGUI.LabelField(rect, "External Assets"),
                elementHeightCallback = (index) => EditorGUIUtility.singleLineHeight,
                drawElementCallback = (rect, index, isActive, focused) =>
                {
                    var itemProperty = _externalAssetsProperty.GetArrayElementAtIndex(index);
                    rect.height = EditorGUIUtility.singleLineHeight;
                    rect = EditorGUI.PrefixLabel(rect, new GUIContent($"Element {index}"));
                    EditorGUI.PropertyField(rect, itemProperty, GUIContent.none);
                },
            };
        }

        public override void OnInspectorGUI()
        {
            EditorGUILayout.BeginVertical();
            GUILayout.Label("Declavatar", Constants.TitleLabel);
            GUILayout.Label("Declarative Avatar Assets Composing Tool, by kb10uy", EditorStyles.centeredGreyMiniLabel);
            EditorGUILayout.Separator();
            EditorGUILayout.EndVertical();

            serializedObject.Update();
            EditorGUILayout.PropertyField(_definitionProperty);
            _externalAssetsList.DoLayoutList();
            serializedObject.ApplyModifiedProperties();
        }
    }
}