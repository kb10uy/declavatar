using System.Collections.Generic;
using UnityEditor;
using UnityEngine;

namespace KusakaFactory.Declavatar
{
    public class DeclavatarWindow : EditorWindow
    {
        private Declavatar _declavatar = null;
        private List<(ErrorKind, string)> _errors = new List<(ErrorKind, string)>();

        private TextAsset _sourceTextAsset = null;
        private GameObject _targetAvatar = null;

        private string _avatarName = "Avatar Name";
        private Animation[] _parameterAnimations = new Animation[3];
        private GameObject[] _parameterObjects = new GameObject[3];

        private Vector2 _windowErrorsScroll = Vector2.zero;
        private bool _windowErrorsShown = false;

        [MenuItem("Window/Declavatar")]
        public static void ShowWindow()
        {
            EditorWindow.GetWindow<DeclavatarWindow>("Declavatar", true);
        }

        public void OnDestroyed()
        {
            _declavatar.Dispose();
        }

        public void OnGUI()
        {
            GetDeclavatar();

            DrawHeader();
            DrawSources();
            DrawGenerators();
            DrawErrorsList();
            DrawNativeDebug();
        }

        private void DrawHeader()
        {
            EditorGUILayout.BeginVertical();
            GUILayout.Label("Declavatar - Declarative Avatar Asset Composing", EditorStyles.largeLabel);
            GUILayout.Label("by kb10uy");
            EditorGUILayout.Separator();
            EditorGUILayout.EndVertical();
        }

        private void DrawSources()
        {
            EditorGUILayout.BeginVertical();
            _sourceTextAsset = (TextAsset)EditorGUILayout.ObjectField(
                "Avatar Declaration",
                _sourceTextAsset,
                typeof(TextAsset),
                false
            );
            _targetAvatar = (GameObject)EditorGUILayout.ObjectField(
                "Target Avatar",
                _targetAvatar,
                typeof(GameObject),
                true
            );
            GUILayout.Button("Compile Declaration", GUILayout.Height(40));
            EditorGUILayout.Separator();
            EditorGUILayout.EndVertical();
        }

        private void DrawGenerators()
        {
            EditorGUILayout.BeginVertical();
            GUILayout.Label("External Animations", EditorStyles.boldLabel);
            for (int i = 0; i < _parameterObjects.Length; i++)
            {
                _parameterAnimations[i] = (Animation)EditorGUILayout.ObjectField(
                    $"Animation {i + 1}",
                    _parameterAnimations[i],
                    typeof(Animation),
                    false
                );
            }

            GUILayout.Label("External GameObjects", EditorStyles.boldLabel);
            for (int i = 0; i < _parameterObjects.Length; i++)
            {
                _parameterObjects[i] = (GameObject)EditorGUILayout.ObjectField(
                    $"GameObject {i + 1}",
                    _parameterObjects[i],
                    typeof(GameObject),
                    true
                );
            }

            GUILayout.Button("Generate Assets", GUILayout.Height(40));
            EditorGUILayout.Separator();
            GUILayout.EndVertical();
        }


        private void DrawErrorsList()
        {
            _windowErrorsShown = EditorGUILayout.Foldout(
                _windowErrorsShown,
                "Declaration Errors"
            );
            if (!_windowErrorsShown) return;

            _windowErrorsScroll = GUILayout.BeginScrollView(_windowErrorsScroll, GUILayout.MaxHeight(200));
            if (_errors.Count > 0)
            {
                var previousBackground = GUI.backgroundColor;
                var iconStyle = new GUIStyle() { padding = new RectOffset(4, 0, 4, 4) };
                foreach (var (kind, message) in _errors)
                {
                    GUILayout.BeginHorizontal();

                    GUIContent icon = null;
                    switch (kind)
                    {
                        case ErrorKind.CompilerError:
                        case ErrorKind.SyntaxError:
                        case ErrorKind.SemanticError:
                            icon = EditorGUIUtility.IconContent("console.erroricon");
                            break;
                        case ErrorKind.SemanticInfo:
                            icon = EditorGUIUtility.IconContent("console.infoicon");
                            break;
                    }
                    GUILayout.Box(icon, iconStyle);

                    GUILayout.BeginVertical();
                    GUILayout.Space(4.0f);
                    GUILayout.Label($"{kind}", EditorStyles.boldLabel);
                    GUILayout.Label(message);
                    GUILayout.EndVertical();

                    GUILayout.FlexibleSpace();
                    GUILayout.EndHorizontal();
                }
                GUI.backgroundColor = previousBackground;
            }
            else
            {
                GUILayout.Label($"No errors detected.");
            }
            GUILayout.EndScrollView();
        }

        private void DrawNativeDebug()
        {
            GUILayout.BeginVertical();
            GUILayout.Label("Native Plugin Debug", EditorStyles.largeLabel);

            if (GUILayout.Button("Check Errors"))
            {
                _declavatar.PushExampleErrors();
                _errors = _declavatar.FetchErrors();
            }
            GUILayout.EndVertical();
        }

        private void GetDeclavatar()
        {
            if (_declavatar != null) return;
            _declavatar = new Declavatar();
        }

        private static class Constants
        {
            public static readonly Color ErrorBackground = new Color(1.0f, 0.2f, 0.2f, 0.4f);
            public static readonly Color ErrorForeground = new Color(1.0f, 0.8f, 0.8f, 1.0f);
            public static readonly Color InfoBackground = new Color(0.2f, 1.0f, 0.2f, 0.4f);
            public static readonly Color InfoForeground = new Color(0.8f, 1.0f, 0.8f, 1.0f);
        }
    }
}
