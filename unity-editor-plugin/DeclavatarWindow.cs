using System.Collections.Generic;
using System.IO;
using System.Linq;
using UnityEditor;
using UnityEngine;
using VRC.SDK3.Avatars.Components;

namespace KusakaFactory.Declavatar
{
    public class DeclavatarWindow : EditorWindow
    {
        private DeclavatarPlugin _declavatarPlugin = null;
        private List<(ErrorKind, string)> _errors = new List<(ErrorKind, string)>();

        private TextAsset _sourceTextAsset = null;
        private VRCAvatarDescriptor _avatarDescriptor = null;
        private string _outputPath = "Assets";

        private string _avatarJson = "";
        private Avatar _avatar = null;
        private Dictionary<string, Material> _materialAssets = new Dictionary<string, Material>();
        private Dictionary<string, AnimationClip> _animationClipAssets = new Dictionary<string, AnimationClip>();

        private Vector2 _windowErrorsScroll = Vector2.zero;

        [MenuItem("Window/Declavatar")]
        public static void ShowWindow()
        {
            EditorWindow.GetWindow<DeclavatarWindow>("Declavatar", true);
        }

        public void OnDestroyed()
        {
            _declavatarPlugin.Dispose();
        }

        public void OnGUI()
        {
            GetDeclavatarPlugin();

            DrawHeader();
            DrawSources();
            DrawStatistics();
            DrawAssets();
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

            EditorGUI.BeginDisabledGroup(_sourceTextAsset == null);
            if (GUILayout.Button("Compile Declaration", GUILayout.Height(40))) Compile();
            EditorGUI.EndDisabledGroup();
            EditorGUILayout.EndVertical();
            EditorGUILayout.Separator();
        }

        private void DrawStatistics()
        {
            if (_avatar == null) return;

            GUILayout.Label("Statistics", Constants.BigBoldLabel);
            EditorGUILayout.BeginVertical(Constants.MarginBox);

            EditorGUILayout.BeginHorizontal();
            EditorGUILayout.LabelField("Avatar Name:", EditorStyles.boldLabel);
            GUILayout.Label($"{_avatar?.Name ?? ""}");
            GUILayout.FlexibleSpace();
            EditorGUILayout.EndHorizontal();

            EditorGUILayout.BeginHorizontal();
            EditorGUILayout.LabelField("Defined Parameters:", EditorStyles.boldLabel);
            GUILayout.Label($"{_avatar?.Parameters.Count ?? 0}");
            GUILayout.FlexibleSpace();
            EditorGUILayout.EndHorizontal();

            EditorGUILayout.BeginHorizontal();
            EditorGUILayout.LabelField("Defined Animation Groups:", EditorStyles.boldLabel);
            GUILayout.Label($"{_avatar?.AnimationGroups.Count ?? 0}");
            GUILayout.FlexibleSpace();
            EditorGUILayout.EndHorizontal();

            EditorGUILayout.BeginHorizontal();
            EditorGUILayout.LabelField("Defined Driver Groups:", EditorStyles.boldLabel);
            GUILayout.Label($"{_avatar?.DriverGroups.Count ?? 0}");
            GUILayout.FlexibleSpace();
            EditorGUILayout.EndHorizontal();

            EditorGUILayout.BeginHorizontal();
            EditorGUILayout.LabelField("Top Menu Items:", EditorStyles.boldLabel);
            GUILayout.Label($"{_avatar?.TopMenuGroup.Items.Count ?? 0}");
            GUILayout.FlexibleSpace();
            EditorGUILayout.EndHorizontal();

            EditorGUILayout.EndVertical();
            EditorGUILayout.Separator();
        }

        private void DrawAssets()
        {
            if (_materialAssets.Count == 0 && _animationClipAssets.Count == 0) return;

            GUILayout.Label("External Assets", Constants.BigBoldLabel);
            EditorGUILayout.BeginVertical(Constants.MarginBox);

            if (_materialAssets.Count != 0)
            {
                var materialKeys = _materialAssets.Keys.ToList();
                foreach (var materialKey in materialKeys)
                {
                    _materialAssets[materialKey] = (Material)EditorGUILayout.ObjectField(
                        materialKey,
                        _materialAssets[materialKey],
                        typeof(Material),
                        false
                    );
                }
                EditorGUILayout.Separator();
            }

            if (_animationClipAssets.Count != 0)
            {
                var animationKeys = _animationClipAssets.Keys.ToList();
                foreach (var animationKey in animationKeys)
                {
                    _animationClipAssets[animationKey] = (AnimationClip)EditorGUILayout.ObjectField(
                        animationKey,
                        _animationClipAssets[animationKey],
                        typeof(AnimationClip),
                        false
                    );
                }
                EditorGUILayout.Separator();
            }

            EditorGUILayout.EndVertical();
        }

        private void DrawGenerators()
        {
            GUILayout.Label("Generation", Constants.BigBoldLabel);
            EditorGUILayout.BeginVertical(Constants.MarginBox);
            _avatarDescriptor = (VRCAvatarDescriptor)EditorGUILayout.ObjectField(
                "Target Avatar",
                _avatarDescriptor,
                typeof(VRCAvatarDescriptor),
                true
            );
            EditorGUILayout.Separator();

            _outputPath = EditorGUILayout.TextField("Output Path", _outputPath);
            EditorGUI.BeginDisabledGroup(_sourceTextAsset == null);
            if (GUILayout.Button("Set to declaration file directory")) SetAutoOutputPath();
            EditorGUI.EndDisabledGroup();
            EditorGUILayout.Separator();

            EditorGUI.BeginDisabledGroup(_avatar != null || _avatarDescriptor == null);
            if (GUILayout.Button("Generate Template Declaration")) GenerateTemplate();
            EditorGUI.EndDisabledGroup();

            EditorGUI.BeginDisabledGroup(_avatar == null || _avatarDescriptor == null);
            if (GUILayout.Button("Generate Assets", GUILayout.Height(40))) GenerateAssets();
            EditorGUI.EndDisabledGroup();

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
                _declavatarPlugin.PushExampleErrors();
                _errors = _declavatarPlugin.FetchErrors();
            }
            GUILayout.EndVertical();
        }

        private void GetDeclavatarPlugin()
        {
            if (_declavatarPlugin != null) return;
            _declavatarPlugin = new DeclavatarPlugin();
        }

        private void Compile()
        {
            if (_declavatarPlugin == null || _sourceTextAsset == null) return;

            _declavatarPlugin.Reset();
            if (_declavatarPlugin.Compile(_sourceTextAsset.text))
            {
                _avatarJson = _declavatarPlugin.GetAvatarJson();
                _avatar = Declavatar.Deserialize(_avatarJson);
                _errors = _declavatarPlugin.FetchErrors();
                UpdateAssetsList();
                SetAutoOutputPath();
                Repaint();
            }
            else
            {
                _avatarJson = "";
                _avatar = null;
                _errors = _declavatarPlugin.FetchErrors();
                UpdateAssetsList();
                Repaint();
            }
        }

        private void UpdateAssetsList()
        {
            _materialAssets.Clear();
            _animationClipAssets.Clear();
            if (_avatar == null) return;

            foreach (var asset in _avatar.Assets)
            {
                switch (asset.AssetType)
                {
                    case "Material":
                        _materialAssets.Add(asset.Key, null);
                        break;
                    case "Animation":
                        _animationClipAssets.Add(asset.Key, null);
                        break;
                }
            }
        }

        private void SetAutoOutputPath()
        {
            if (_sourceTextAsset == null) return;
            _outputPath = Path.GetDirectoryName(AssetDatabase.GetAssetPath(_sourceTextAsset));
            Repaint();
        }

        private void GenerateTemplate()
        {
            if (_avatarDescriptor == null) return;

            var externals = new ExternalAssets { Materials = _materialAssets, AnimationClips = _animationClipAssets };
            var declavatar = new Declavatar(_avatar, externals, _avatarDescriptor, _outputPath);
            declavatar.GenerateTemplateTextAsset();
        }

        private void GenerateAssets()
        {
            if (_avatar == null || _avatarDescriptor == null) return;

            var externals = new ExternalAssets { Materials = _materialAssets, AnimationClips = _animationClipAssets };
            var declavatar = new Declavatar(_avatar, externals, _avatarDescriptor, _outputPath);
            declavatar.GenerateAllAssets();
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
