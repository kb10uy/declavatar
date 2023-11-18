using UnityEditor;
using UnityEngine;

namespace KusakaFactory.Declavatar.Editor
{
    internal static class Constants
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
