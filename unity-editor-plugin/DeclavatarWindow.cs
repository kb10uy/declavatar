using System.Collections.Generic;
using UnityEditor;
using UnityEngine;

namespace KusakaFactory.Declavatar
{
    public class DeclavatarWindow : EditorWindow
    {
        private DeclavatarPlugin _declavatar = null;
        private List<(ErrorKind, string)> _errors = new List<(ErrorKind, string)>();

        private TextAsset _sourceTextAsset = null;
        private GameObject _targetAvatar = null;

        private string _avatarJson = "";
        private Avatar _avatarDefinition = null;
        private Animation[] _parameterAnimations = new Animation[3];
        private GameObject[] _parameterObjects = new GameObject[3];

        private Vector2 _windowErrorsScroll = Vector2.zero;

        private void Compile()
        {
            if (_declavatar == null) return;
            if (_sourceTextAsset == null) return;

            _declavatar.Reset();
            if (_declavatar.Compile(_sourceTextAsset.text))
            {
                _avatarJson = _declavatar.GetAvatarJson();
                _avatarDefinition = Declavatar.Deserialize(_avatarJson);
                _errors = _declavatar.FetchErrors();
                Repaint();
            }
            else
            {
                _avatarJson = "";
                _errors = _declavatar.FetchErrors();
                Repaint();
            }
        }

        private void GenerateAssets()
        {
            if (_avatarDefinition == null) return;

            Declavatar.GenerateParametersAsset(_avatarDefinition, "Assets", "test.asset");
        }

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
            DrawStatistics();
            DrawGenerators();
            DrawErrorsList();
            // DrawNativeDebug();
        }

        private void DrawHeader()
        {
            EditorGUILayout.BeginVertical();
            GUILayout.Label("Declavatar", Constants.TitleLabel);
            GUILayout.Label("Declarative Avatar Assets Composing Tool, by kb10uy", EditorStyles.centeredGreyMiniLabel);
            EditorGUILayout.Separator();
            EditorGUILayout.EndVertical();
        }

        private void DrawSources()
        {
            GUILayout.Label("Compile", Constants.BigBoldLabel);
            EditorGUILayout.BeginVertical(Constants.MarginBox);
            _sourceTextAsset = (TextAsset)EditorGUILayout.ObjectField(
                "Avatar Declaration",
                _sourceTextAsset,
                typeof(TextAsset),
                false
            );
            EditorGUILayout.Separator();

            if (GUILayout.Button("Compile Declaration", GUILayout.Height(40)))
            {
                Compile();
            }
            EditorGUILayout.EndVertical();
            EditorGUILayout.Separator();
        }

        private void DrawStatistics()
        {
            GUILayout.Label("Statistics", Constants.BigBoldLabel);
            EditorGUILayout.BeginVertical(Constants.MarginBox);

            EditorGUILayout.BeginHorizontal();
            EditorGUILayout.LabelField("Avatar Name:", EditorStyles.boldLabel);
            GUILayout.Label($"{_avatarDefinition?.Name ?? ""}");
            GUILayout.FlexibleSpace();
            EditorGUILayout.EndHorizontal();

            EditorGUILayout.BeginHorizontal();
            EditorGUILayout.LabelField("Defined Parameters:", EditorStyles.boldLabel);
            GUILayout.Label($"{_avatarDefinition?.Parameters.Count ?? 0}");
            GUILayout.FlexibleSpace();
            EditorGUILayout.EndHorizontal();

            EditorGUILayout.BeginHorizontal();
            EditorGUILayout.LabelField("Defined Animation Groups:", EditorStyles.boldLabel);
            GUILayout.Label($"{_avatarDefinition?.AnimationGroups.Count ?? 0}");
            GUILayout.FlexibleSpace();
            EditorGUILayout.EndHorizontal();

            EditorGUILayout.BeginHorizontal();
            EditorGUILayout.LabelField("Defined Driver Groups:", EditorStyles.boldLabel);
            GUILayout.Label($"{_avatarDefinition?.DriverGroups.Count ?? 0}");
            GUILayout.FlexibleSpace();
            EditorGUILayout.EndHorizontal();

            EditorGUILayout.BeginHorizontal();
            EditorGUILayout.LabelField("Top Menu Items:", EditorStyles.boldLabel);
            GUILayout.Label($"{_avatarDefinition?.TopMenuGroup.Items.Count ?? 0}");
            GUILayout.FlexibleSpace();
            EditorGUILayout.EndHorizontal();

            EditorGUILayout.EndVertical();
            EditorGUILayout.Separator();
        }

        private void DrawGenerators()
        {
            GUILayout.Label("Generation", Constants.BigBoldLabel);
            EditorGUILayout.BeginVertical(Constants.MarginBox);
            _targetAvatar = (GameObject)EditorGUILayout.ObjectField(
                "Target Avatar",
                _targetAvatar,
                typeof(GameObject),
                true
            );
            EditorGUILayout.Separator();

            for (int i = 0; i < _parameterObjects.Length; i++)
            {
                _parameterAnimations[i] = (Animation)EditorGUILayout.ObjectField(
                    $"Animation {i + 1}",
                    _parameterAnimations[i],
                    typeof(Animation),
                    false
                );
            }
            EditorGUILayout.Separator();

            for (int i = 0; i < _parameterObjects.Length; i++)
            {
                _parameterObjects[i] = (GameObject)EditorGUILayout.ObjectField(
                    $"GameObject {i + 1}",
                    _parameterObjects[i],
                    typeof(GameObject),
                    true
                );
            }
            EditorGUILayout.Separator();

            if (GUILayout.Button("Generate Assets", GUILayout.Height(40)))
            {
                GenerateAssets();
            }
            GUILayout.EndVertical();
            EditorGUILayout.Separator();
        }


        private void DrawErrorsList()
        {
            GUILayout.Label("Errors", Constants.BigBoldLabel);
            _windowErrorsScroll = GUILayout.BeginScrollView(_windowErrorsScroll, Constants.MarginBox, GUILayout.MaxHeight(200));
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
            _declavatar = new DeclavatarPlugin();
        }

        private static class Constants
        {
            public static readonly Color ErrorBackground = new Color(1.0f, 0.2f, 0.2f, 0.4f);
            public static readonly Color ErrorForeground = new Color(1.0f, 0.8f, 0.8f, 1.0f);
            public static readonly Color InfoBackground = new Color(0.2f, 1.0f, 0.2f, 0.4f);
            public static readonly Color InfoForeground = new Color(0.8f, 1.0f, 0.8f, 1.0f);
            public static GUIStyle MarginBox { get; private set; }
            public static GUIStyle BigBoldLabel { get; private set; }
            public static GUIStyle TitleLabel { get; private set; }

            static Constants()
            {
                MarginBox = new GUIStyle()
                {
                    margin = new RectOffset(4, 4, 4, 4),
                };
                BigBoldLabel = new GUIStyle(EditorStyles.boldLabel)
                {
                    fontSize = 14,
                };
                TitleLabel = new GUIStyle(EditorStyles.boldLabel)
                {
                    fontSize = 24,
                    alignment = TextAnchor.MiddleCenter,
                    margin = new RectOffset(8, 8, 4, 4),
                };
            }
        }
    }
}
