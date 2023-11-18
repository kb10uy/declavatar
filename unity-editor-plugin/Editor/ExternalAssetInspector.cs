using UnityEditor;
using UnityEditorInternal;
using UnityEngine;

namespace KusakaFactory.Declavatar.Editor
{
    [CustomEditor(typeof(ExternalAsset))]
    public sealed class ExternalAssetInspector : UnityEditor.Editor
    {
        private SerializedProperty _animationsProperty;
        private SerializedProperty _materialsProperty;
        private SerializedProperty _localizationsProperty;

        private ReorderableList _animationsList;
        private ReorderableList _materialsList;
        private ReorderableList _localizationsList;

        public void OnEnable()
        {
            _animationsProperty = serializedObject.FindProperty(nameof(ExternalAsset.Animations));
            _materialsProperty = serializedObject.FindProperty(nameof(ExternalAsset.Materials));
            _localizationsProperty = serializedObject.FindProperty(nameof(ExternalAsset.Localizations));

            _animationsList = new ReorderableList(serializedObject, _animationsProperty)
            {
                drawHeaderCallback = (rect) => EditorGUI.LabelField(rect, "Animation Clip Definitions"),
                elementHeightCallback = (index) => EditorGUIUtility.singleLineHeight * 2 + 8,
                drawElementCallback = (rect, index, isActive, focused) =>
                {
                    var itemProperty = _animationsProperty.GetArrayElementAtIndex(index);

                    rect.height = EditorGUIUtility.singleLineHeight;
                    rect = EditorGUI.PrefixLabel(rect, new GUIContent($"Element {index}"));
                    var keyRect = new Rect(rect.x, rect.y + 2, rect.width, rect.height);
                    EditorGUI.PropertyField(keyRect, itemProperty.FindPropertyRelative("Key"), GUIContent.none);
                    var valueRect = new Rect(rect.x, rect.y + rect.height + 6, rect.width, rect.height);
                    EditorGUI.PropertyField(valueRect, itemProperty.FindPropertyRelative("Animation"), GUIContent.none);
                },
            };

            _materialsList = new ReorderableList(serializedObject, _materialsProperty)
            {
                drawHeaderCallback = (rect) => EditorGUI.LabelField(rect, "Material Definitions"),
                elementHeightCallback = (index) => EditorGUIUtility.singleLineHeight * 2 + 8,
                drawElementCallback = (rect, index, isActive, focused) =>
                {
                    var itemProperty = _materialsProperty.GetArrayElementAtIndex(index);

                    rect.height = EditorGUIUtility.singleLineHeight;
                    rect = EditorGUI.PrefixLabel(rect, new GUIContent($"Element {index}"));
                    var keyRect = new Rect(rect.x, rect.y + 2, rect.width, rect.height);
                    EditorGUI.PropertyField(keyRect, itemProperty.FindPropertyRelative("Key"), GUIContent.none);
                    var valueRect = new Rect(rect.x, rect.y + rect.height + 6, rect.width, rect.height);
                    EditorGUI.PropertyField(valueRect, itemProperty.FindPropertyRelative("Material"), GUIContent.none);
                },
            };

            _localizationsList = new ReorderableList(serializedObject, _localizationsProperty)
            {
                drawHeaderCallback = (rect) => EditorGUI.LabelField(rect, "Localization Definitions"),
                elementHeightCallback = (index) => EditorGUIUtility.singleLineHeight * 2 + 8,
                drawElementCallback = (rect, index, isActive, focused) =>
                {
                    var itemProperty = _localizationsProperty.GetArrayElementAtIndex(index);

                    rect.height = EditorGUIUtility.singleLineHeight;
                    rect = EditorGUI.PrefixLabel(rect, new GUIContent($"Element {index}"));
                    var keyRect = new Rect(rect.x, rect.y + 2, rect.width, rect.height);
                    EditorGUI.PropertyField(keyRect, itemProperty.FindPropertyRelative("Key"), GUIContent.none);
                    var valueRect = new Rect(rect.x, rect.y + rect.height + 6, rect.width, rect.height);
                    EditorGUI.PropertyField(valueRect, itemProperty.FindPropertyRelative("Localization"), GUIContent.none);
                },
            };
        }

        public override void OnInspectorGUI()
        {
            serializedObject.Update();

            _animationsList.DoLayoutList();
            _materialsList.DoLayoutList();
            _localizationsList.DoLayoutList();

            serializedObject.ApplyModifiedProperties();
        }
    }
}
