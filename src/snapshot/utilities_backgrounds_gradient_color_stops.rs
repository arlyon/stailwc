use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`gradientColorStops`"#####, r#####"({
  inherit: "inherit",
  current: "currentColor",
  transparent: "transparent",
  black: "#000",
  white: "#fff",
  slate: {
    50: "#f8fafc",
    100: "#f1f5f9",
    200: "#e2e8f0",
    300: "#cbd5e1",
    400: "#94a3b8",
    500: "#64748b",
    600: "#475569",
    700: "#334155",
    800: "#1e293b",
    900: "#0f172a",
    950: "#020617",
  },
  gray: {
    50: "#f9fafb",
    100: "#f3f4f6",
    200: "#e5e7eb",
    300: "#d1d5db",
    400: "#9ca3af",
    500: "#6b7280",
    600: "#4b5563",
    700: "#374151",
    800: "#1f2937",
    900: "#111827",
    950: "#030712",
  },
  zinc: {
    50: "#fafafa",
    100: "#f4f4f5",
    200: "#e4e4e7",
    300: "#d4d4d8",
    400: "#a1a1aa",
    500: "#71717a",
    600: "#52525b",
    700: "#3f3f46",
    800: "#27272a",
    900: "#18181b",
    950: "#09090b",
  },
  neutral: {
    50: "#fafafa",
    100: "#f5f5f5",
    200: "#e5e5e5",
    300: "#d4d4d4",
    400: "#a3a3a3",
    500: "#737373",
    600: "#525252",
    700: "#404040",
    800: "#262626",
    900: "#171717",
    950: "#0a0a0a",
  },
  stone: {
    50: "#fafaf9",
    100: "#f5f5f4",
    200: "#e7e5e4",
    300: "#d6d3d1",
    400: "#a8a29e",
    500: "#78716c",
    600: "#57534e",
    700: "#44403c",
    800: "#292524",
    900: "#1c1917",
    950: "#0c0a09",
  },
  red: {
    50: "#fef2f2",
    100: "#fee2e2",
    200: "#fecaca",
    300: "#fca5a5",
    400: "#f87171",
    500: "#ef4444",
    600: "#dc2626",
    700: "#b91c1c",
    800: "#991b1b",
    900: "#7f1d1d",
    950: "#450a0a",
  },
  orange: {
    50: "#fff7ed",
    100: "#ffedd5",
    200: "#fed7aa",
    300: "#fdba74",
    400: "#fb923c",
    500: "#f97316",
    600: "#ea580c",
    700: "#c2410c",
    800: "#9a3412",
    900: "#7c2d12",
    950: "#431407",
  },
  amber: {
    50: "#fffbeb",
    100: "#fef3c7",
    200: "#fde68a",
    300: "#fcd34d",
    400: "#fbbf24",
    500: "#f59e0b",
    600: "#d97706",
    700: "#b45309",
    800: "#92400e",
    900: "#78350f",
    950: "#451a03",
  },
  yellow: {
    50: "#fefce8",
    100: "#fef9c3",
    200: "#fef08a",
    300: "#fde047",
    400: "#facc15",
    500: "#eab308",
    600: "#ca8a04",
    700: "#a16207",
    800: "#854d0e",
    900: "#713f12",
    950: "#422006",
  },
  lime: {
    50: "#f7fee7",
    100: "#ecfccb",
    200: "#d9f99d",
    300: "#bef264",
    400: "#a3e635",
    500: "#84cc16",
    600: "#65a30d",
    700: "#4d7c0f",
    800: "#3f6212",
    900: "#365314",
    950: "#1a2e05",
  },
  green: {
    50: "#f0fdf4",
    100: "#dcfce7",
    200: "#bbf7d0",
    300: "#86efac",
    400: "#4ade80",
    500: "#22c55e",
    600: "#16a34a",
    700: "#15803d",
    800: "#166534",
    900: "#14532d",
    950: "#052e16",
  },
  emerald: {
    50: "#ecfdf5",
    100: "#d1fae5",
    200: "#a7f3d0",
    300: "#6ee7b7",
    400: "#34d399",
    500: "#10b981",
    600: "#059669",
    700: "#047857",
    800: "#065f46",
    900: "#064e3b",
    950: "#022c22",
  },
  teal: {
    50: "#f0fdfa",
    100: "#ccfbf1",
    200: "#99f6e4",
    300: "#5eead4",
    400: "#2dd4bf",
    500: "#14b8a6",
    600: "#0d9488",
    700: "#0f766e",
    800: "#115e59",
    900: "#134e4a",
    950: "#042f2e",
  },
  cyan: {
    50: "#ecfeff",
    100: "#cffafe",
    200: "#a5f3fc",
    300: "#67e8f9",
    400: "#22d3ee",
    500: "#06b6d4",
    600: "#0891b2",
    700: "#0e7490",
    800: "#155e75",
    900: "#164e63",
    950: "#083344",
  },
  sky: {
    50: "#f0f9ff",
    100: "#e0f2fe",
    200: "#bae6fd",
    300: "#7dd3fc",
    400: "#38bdf8",
    500: "#0ea5e9",
    600: "#0284c7",
    700: "#0369a1",
    800: "#075985",
    900: "#0c4a6e",
    950: "#082f49",
  },
  blue: {
    50: "#eff6ff",
    100: "#dbeafe",
    200: "#bfdbfe",
    300: "#93c5fd",
    400: "#60a5fa",
    500: "#3b82f6",
    600: "#2563eb",
    700: "#1d4ed8",
    800: "#1e40af",
    900: "#1e3a8a",
    950: "#172554",
  },
  indigo: {
    50: "#eef2ff",
    100: "#e0e7ff",
    200: "#c7d2fe",
    300: "#a5b4fc",
    400: "#818cf8",
    500: "#6366f1",
    600: "#4f46e5",
    700: "#4338ca",
    800: "#3730a3",
    900: "#312e81",
    950: "#1e1b4b",
  },
  violet: {
    50: "#f5f3ff",
    100: "#ede9fe",
    200: "#ddd6fe",
    300: "#c4b5fd",
    400: "#a78bfa",
    500: "#8b5cf6",
    600: "#7c3aed",
    700: "#6d28d9",
    800: "#5b21b6",
    900: "#4c1d95",
    950: "#2e1065",
  },
  purple: {
    50: "#faf5ff",
    100: "#f3e8ff",
    200: "#e9d5ff",
    300: "#d8b4fe",
    400: "#c084fc",
    500: "#a855f7",
    600: "#9333ea",
    700: "#7e22ce",
    800: "#6b21a8",
    900: "#581c87",
    950: "#3b0764",
  },
  fuchsia: {
    50: "#fdf4ff",
    100: "#fae8ff",
    200: "#f5d0fe",
    300: "#f0abfc",
    400: "#e879f9",
    500: "#d946ef",
    600: "#c026d3",
    700: "#a21caf",
    800: "#86198f",
    900: "#701a75",
    950: "#4a044e",
  },
  pink: {
    50: "#fdf2f8",
    100: "#fce7f3",
    200: "#fbcfe8",
    300: "#f9a8d4",
    400: "#f472b6",
    500: "#ec4899",
    600: "#db2777",
    700: "#be185d",
    800: "#9d174d",
    900: "#831843",
    950: "#500724",
  },
  rose: {
    50: "#fff1f2",
    100: "#ffe4e6",
    200: "#fecdd3",
    300: "#fda4af",
    400: "#fb7185",
    500: "#f43f5e",
    600: "#e11d48",
    700: "#be123c",
    800: "#9f1239",
    900: "#881337",
    950: "#4c0519",
  },
  'red-500/fromConfig': "#000",
  electric: void 0,
})
;"##### ; "0")]
#[test_case(r#####"tw`from-inherit`"#####, r#####"({
  '--tw-gradient-from': "inherit var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 255 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "1")]
#[test_case(r#####"tw`from-current`"#####, r#####"({
  '--tw-gradient-from': "currentColor var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 255 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "2")]
#[test_case(r#####"tw`from-transparent`"#####, r#####"({
  '--tw-gradient-from': "transparent var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(0 0 0 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "3")]
#[test_case(r#####"tw`from-black`"#####, r#####"({
  '--tw-gradient-from': "#000 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(0 0 0 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "4")]
#[test_case(r#####"tw`from-white`"#####, r#####"({
  '--tw-gradient-from': "#fff var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 255 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "5")]
#[test_case(r#####"tw`from-slate-50`"#####, r#####"({
  '--tw-gradient-from': "#f8fafc var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(248 250 252 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "6")]
#[test_case(r#####"tw`from-slate-100`"#####, r#####"({
  '--tw-gradient-from': "#f1f5f9 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(241 245 249 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "7")]
#[test_case(r#####"tw`from-slate-200`"#####, r#####"({
  '--tw-gradient-from': "#e2e8f0 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(226 232 240 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "8")]
#[test_case(r#####"tw`from-slate-300`"#####, r#####"({
  '--tw-gradient-from': "#cbd5e1 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(203 213 225 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "9")]
#[test_case(r#####"tw`from-slate-400`"#####, r#####"({
  '--tw-gradient-from': "#94a3b8 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(148 163 184 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "10")]
#[test_case(r#####"tw`from-slate-500`"#####, r#####"({
  '--tw-gradient-from': "#64748b var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(100 116 139 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "11")]
#[test_case(r#####"tw`from-slate-600`"#####, r#####"({
  '--tw-gradient-from': "#475569 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(71 85 105 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "12")]
#[test_case(r#####"tw`from-slate-700`"#####, r#####"({
  '--tw-gradient-from': "#334155 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(51 65 85 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "13")]
#[test_case(r#####"tw`from-slate-800`"#####, r#####"({
  '--tw-gradient-from': "#1e293b var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(30 41 59 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "14")]
#[test_case(r#####"tw`from-slate-900`"#####, r#####"({
  '--tw-gradient-from': "#0f172a var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(15 23 42 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "15")]
#[test_case(r#####"tw`from-gray-50`"#####, r#####"({
  '--tw-gradient-from': "#f9fafb var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(249 250 251 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "16")]
#[test_case(r#####"tw`from-gray-100`"#####, r#####"({
  '--tw-gradient-from': "#f3f4f6 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(243 244 246 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "17")]
#[test_case(r#####"tw`from-gray-200`"#####, r#####"({
  '--tw-gradient-from': "#e5e7eb var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(229 231 235 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "18")]
#[test_case(r#####"tw`from-gray-300`"#####, r#####"({
  '--tw-gradient-from': "#d1d5db var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(209 213 219 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "19")]
#[test_case(r#####"tw`from-gray-400`"#####, r#####"({
  '--tw-gradient-from': "#9ca3af var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(156 163 175 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "20")]
#[test_case(r#####"tw`from-gray-500`"#####, r#####"({
  '--tw-gradient-from': "#6b7280 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(107 114 128 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "21")]
#[test_case(r#####"tw`from-gray-600`"#####, r#####"({
  '--tw-gradient-from': "#4b5563 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(75 85 99 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "22")]
#[test_case(r#####"tw`from-gray-700`"#####, r#####"({
  '--tw-gradient-from': "#374151 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(55 65 81 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "23")]
#[test_case(r#####"tw`from-gray-800`"#####, r#####"({
  '--tw-gradient-from': "#1f2937 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(31 41 55 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "24")]
#[test_case(r#####"tw`from-gray-900`"#####, r#####"({
  '--tw-gradient-from': "#111827 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(17 24 39 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "25")]
#[test_case(r#####"tw`from-zinc-50`"#####, r#####"({
  '--tw-gradient-from': "#fafafa var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(250 250 250 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "26")]
#[test_case(r#####"tw`from-zinc-100`"#####, r#####"({
  '--tw-gradient-from': "#f4f4f5 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(244 244 245 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "27")]
#[test_case(r#####"tw`from-zinc-200`"#####, r#####"({
  '--tw-gradient-from': "#e4e4e7 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(228 228 231 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "28")]
#[test_case(r#####"tw`from-zinc-300`"#####, r#####"({
  '--tw-gradient-from': "#d4d4d8 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(212 212 216 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "29")]
#[test_case(r#####"tw`from-zinc-400`"#####, r#####"({
  '--tw-gradient-from': "#a1a1aa var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(161 161 170 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "30")]
#[test_case(r#####"tw`from-zinc-500`"#####, r#####"({
  '--tw-gradient-from': "#71717a var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(113 113 122 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "31")]
#[test_case(r#####"tw`from-zinc-600`"#####, r#####"({
  '--tw-gradient-from': "#52525b var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(82 82 91 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "32")]
#[test_case(r#####"tw`from-zinc-700`"#####, r#####"({
  '--tw-gradient-from': "#3f3f46 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(63 63 70 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "33")]
#[test_case(r#####"tw`from-zinc-800`"#####, r#####"({
  '--tw-gradient-from': "#27272a var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(39 39 42 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "34")]
#[test_case(r#####"tw`from-zinc-900`"#####, r#####"({
  '--tw-gradient-from': "#18181b var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(24 24 27 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "35")]
#[test_case(r#####"tw`from-neutral-50`"#####, r#####"({
  '--tw-gradient-from': "#fafafa var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(250 250 250 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "36")]
#[test_case(r#####"tw`from-neutral-100`"#####, r#####"({
  '--tw-gradient-from': "#f5f5f5 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(245 245 245 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "37")]
#[test_case(r#####"tw`from-neutral-200`"#####, r#####"({
  '--tw-gradient-from': "#e5e5e5 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(229 229 229 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "38")]
#[test_case(r#####"tw`from-neutral-300`"#####, r#####"({
  '--tw-gradient-from': "#d4d4d4 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(212 212 212 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "39")]
#[test_case(r#####"tw`from-neutral-400`"#####, r#####"({
  '--tw-gradient-from': "#a3a3a3 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(163 163 163 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "40")]
#[test_case(r#####"tw`from-neutral-500`"#####, r#####"({
  '--tw-gradient-from': "#737373 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(115 115 115 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "41")]
#[test_case(r#####"tw`from-neutral-600`"#####, r#####"({
  '--tw-gradient-from': "#525252 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(82 82 82 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "42")]
#[test_case(r#####"tw`from-neutral-700`"#####, r#####"({
  '--tw-gradient-from': "#404040 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(64 64 64 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "43")]
#[test_case(r#####"tw`from-neutral-800`"#####, r#####"({
  '--tw-gradient-from': "#262626 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(38 38 38 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "44")]
#[test_case(r#####"tw`from-neutral-900`"#####, r#####"({
  '--tw-gradient-from': "#171717 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(23 23 23 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "45")]
#[test_case(r#####"tw`from-stone-50`"#####, r#####"({
  '--tw-gradient-from': "#fafaf9 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(250 250 249 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "46")]
#[test_case(r#####"tw`from-stone-100`"#####, r#####"({
  '--tw-gradient-from': "#f5f5f4 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(245 245 244 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "47")]
#[test_case(r#####"tw`from-stone-200`"#####, r#####"({
  '--tw-gradient-from': "#e7e5e4 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(231 229 228 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "48")]
#[test_case(r#####"tw`from-stone-300`"#####, r#####"({
  '--tw-gradient-from': "#d6d3d1 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(214 211 209 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "49")]
#[test_case(r#####"tw`from-stone-400`"#####, r#####"({
  '--tw-gradient-from': "#a8a29e var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(168 162 158 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "50")]
#[test_case(r#####"tw`from-stone-500`"#####, r#####"({
  '--tw-gradient-from': "#78716c var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(120 113 108 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "51")]
#[test_case(r#####"tw`from-stone-600`"#####, r#####"({
  '--tw-gradient-from': "#57534e var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(87 83 78 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "52")]
#[test_case(r#####"tw`from-stone-700`"#####, r#####"({
  '--tw-gradient-from': "#44403c var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(68 64 60 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "53")]
#[test_case(r#####"tw`from-stone-800`"#####, r#####"({
  '--tw-gradient-from': "#292524 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(41 37 36 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "54")]
#[test_case(r#####"tw`from-stone-900`"#####, r#####"({
  '--tw-gradient-from': "#1c1917 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(28 25 23 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "55")]
#[test_case(r#####"tw`from-red-50`"#####, r#####"({
  '--tw-gradient-from': "#fef2f2 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 242 242 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "56")]
#[test_case(r#####"tw`from-red-100`"#####, r#####"({
  '--tw-gradient-from': "#fee2e2 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 226 226 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "57")]
#[test_case(r#####"tw`from-red-200`"#####, r#####"({
  '--tw-gradient-from': "#fecaca var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 202 202 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "58")]
#[test_case(r#####"tw`from-red-300`"#####, r#####"({
  '--tw-gradient-from': "#fca5a5 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(252 165 165 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "59")]
#[test_case(r#####"tw`from-red-400`"#####, r#####"({
  '--tw-gradient-from': "#f87171 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(248 113 113 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "60")]
#[test_case(r#####"tw`from-red-500`"#####, r#####"({
  '--tw-gradient-from': "#ef4444 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 68 68 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "61")]
#[test_case(r#####"tw`from-red-600`"#####, r#####"({
  '--tw-gradient-from': "#dc2626 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(220 38 38 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "62")]
#[test_case(r#####"tw`from-red-700`"#####, r#####"({
  '--tw-gradient-from': "#b91c1c var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(185 28 28 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "63")]
#[test_case(r#####"tw`from-red-800`"#####, r#####"({
  '--tw-gradient-from': "#991b1b var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(153 27 27 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "64")]
#[test_case(r#####"tw`from-red-900`"#####, r#####"({
  '--tw-gradient-from': "#7f1d1d var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(127 29 29 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "65")]
#[test_case(r#####"tw`from-orange-50`"#####, r#####"({
  '--tw-gradient-from': "#fff7ed var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 247 237 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "66")]
#[test_case(r#####"tw`from-orange-100`"#####, r#####"({
  '--tw-gradient-from': "#ffedd5 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 237 213 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "67")]
#[test_case(r#####"tw`from-orange-200`"#####, r#####"({
  '--tw-gradient-from': "#fed7aa var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 215 170 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "68")]
#[test_case(r#####"tw`from-orange-300`"#####, r#####"({
  '--tw-gradient-from': "#fdba74 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(253 186 116 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "69")]
#[test_case(r#####"tw`from-orange-400`"#####, r#####"({
  '--tw-gradient-from': "#fb923c var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(251 146 60 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "70")]
#[test_case(r#####"tw`from-orange-500`"#####, r#####"({
  '--tw-gradient-from': "#f97316 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(249 115 22 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "71")]
#[test_case(r#####"tw`from-orange-600`"#####, r#####"({
  '--tw-gradient-from': "#ea580c var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(234 88 12 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "72")]
#[test_case(r#####"tw`from-orange-700`"#####, r#####"({
  '--tw-gradient-from': "#c2410c var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(194 65 12 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "73")]
#[test_case(r#####"tw`from-orange-800`"#####, r#####"({
  '--tw-gradient-from': "#9a3412 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(154 52 18 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "74")]
#[test_case(r#####"tw`from-orange-900`"#####, r#####"({
  '--tw-gradient-from': "#7c2d12 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(124 45 18 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "75")]
#[test_case(r#####"tw`from-amber-50`"#####, r#####"({
  '--tw-gradient-from': "#fffbeb var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 251 235 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "76")]
#[test_case(r#####"tw`from-amber-100`"#####, r#####"({
  '--tw-gradient-from': "#fef3c7 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 243 199 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "77")]
#[test_case(r#####"tw`from-amber-200`"#####, r#####"({
  '--tw-gradient-from': "#fde68a var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(253 230 138 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "78")]
#[test_case(r#####"tw`from-amber-300`"#####, r#####"({
  '--tw-gradient-from': "#fcd34d var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(252 211 77 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "79")]
#[test_case(r#####"tw`from-amber-400`"#####, r#####"({
  '--tw-gradient-from': "#fbbf24 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(251 191 36 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "80")]
#[test_case(r#####"tw`from-amber-500`"#####, r#####"({
  '--tw-gradient-from': "#f59e0b var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(245 158 11 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "81")]
#[test_case(r#####"tw`from-amber-600`"#####, r#####"({
  '--tw-gradient-from': "#d97706 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(217 119 6 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "82")]
#[test_case(r#####"tw`from-amber-700`"#####, r#####"({
  '--tw-gradient-from': "#b45309 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(180 83 9 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "83")]
#[test_case(r#####"tw`from-amber-800`"#####, r#####"({
  '--tw-gradient-from': "#92400e var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(146 64 14 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "84")]
#[test_case(r#####"tw`from-amber-900`"#####, r#####"({
  '--tw-gradient-from': "#78350f var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(120 53 15 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "85")]
#[test_case(r#####"tw`from-yellow-50`"#####, r#####"({
  '--tw-gradient-from': "#fefce8 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 252 232 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "86")]
#[test_case(r#####"tw`from-yellow-100`"#####, r#####"({
  '--tw-gradient-from': "#fef9c3 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 249 195 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "87")]
#[test_case(r#####"tw`from-yellow-200`"#####, r#####"({
  '--tw-gradient-from': "#fef08a var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 240 138 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "88")]
#[test_case(r#####"tw`from-yellow-300`"#####, r#####"({
  '--tw-gradient-from': "#fde047 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(253 224 71 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "89")]
#[test_case(r#####"tw`from-yellow-400`"#####, r#####"({
  '--tw-gradient-from': "#facc15 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(250 204 21 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "90")]
#[test_case(r#####"tw`from-yellow-500`"#####, r#####"({
  '--tw-gradient-from': "#eab308 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(234 179 8 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "91")]
#[test_case(r#####"tw`from-yellow-600`"#####, r#####"({
  '--tw-gradient-from': "#ca8a04 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(202 138 4 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "92")]
#[test_case(r#####"tw`from-yellow-700`"#####, r#####"({
  '--tw-gradient-from': "#a16207 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(161 98 7 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "93")]
#[test_case(r#####"tw`from-yellow-800`"#####, r#####"({
  '--tw-gradient-from': "#854d0e var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(133 77 14 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "94")]
#[test_case(r#####"tw`from-yellow-900`"#####, r#####"({
  '--tw-gradient-from': "#713f12 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(113 63 18 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "95")]
#[test_case(r#####"tw`from-lime-50`"#####, r#####"({
  '--tw-gradient-from': "#f7fee7 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(247 254 231 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "96")]
#[test_case(r#####"tw`from-lime-100`"#####, r#####"({
  '--tw-gradient-from': "#ecfccb var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(236 252 203 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "97")]
#[test_case(r#####"tw`from-lime-200`"#####, r#####"({
  '--tw-gradient-from': "#d9f99d var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(217 249 157 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "98")]
#[test_case(r#####"tw`from-lime-300`"#####, r#####"({
  '--tw-gradient-from': "#bef264 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(190 242 100 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "99")]
#[test_case(r#####"tw`from-lime-400`"#####, r#####"({
  '--tw-gradient-from': "#a3e635 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(163 230 53 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "100")]
#[test_case(r#####"tw`from-lime-500`"#####, r#####"({
  '--tw-gradient-from': "#84cc16 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(132 204 22 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "101")]
#[test_case(r#####"tw`from-lime-600`"#####, r#####"({
  '--tw-gradient-from': "#65a30d var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(101 163 13 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "102")]
#[test_case(r#####"tw`from-lime-700`"#####, r#####"({
  '--tw-gradient-from': "#4d7c0f var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(77 124 15 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "103")]
#[test_case(r#####"tw`from-lime-800`"#####, r#####"({
  '--tw-gradient-from': "#3f6212 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(63 98 18 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "104")]
#[test_case(r#####"tw`from-lime-900`"#####, r#####"({
  '--tw-gradient-from': "#365314 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(54 83 20 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "105")]
#[test_case(r#####"tw`from-green-50`"#####, r#####"({
  '--tw-gradient-from': "#f0fdf4 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(240 253 244 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "106")]
#[test_case(r#####"tw`from-green-100`"#####, r#####"({
  '--tw-gradient-from': "#dcfce7 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(220 252 231 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "107")]
#[test_case(r#####"tw`from-green-200`"#####, r#####"({
  '--tw-gradient-from': "#bbf7d0 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(187 247 208 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "108")]
#[test_case(r#####"tw`from-green-300`"#####, r#####"({
  '--tw-gradient-from': "#86efac var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(134 239 172 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "109")]
#[test_case(r#####"tw`from-green-400`"#####, r#####"({
  '--tw-gradient-from': "#4ade80 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(74 222 128 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "110")]
#[test_case(r#####"tw`from-green-500`"#####, r#####"({
  '--tw-gradient-from': "#22c55e var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(34 197 94 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "111")]
#[test_case(r#####"tw`from-green-600`"#####, r#####"({
  '--tw-gradient-from': "#16a34a var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(22 163 74 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "112")]
#[test_case(r#####"tw`from-green-700`"#####, r#####"({
  '--tw-gradient-from': "#15803d var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(21 128 61 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "113")]
#[test_case(r#####"tw`from-green-800`"#####, r#####"({
  '--tw-gradient-from': "#166534 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(22 101 52 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "114")]
#[test_case(r#####"tw`from-green-900`"#####, r#####"({
  '--tw-gradient-from': "#14532d var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(20 83 45 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "115")]
#[test_case(r#####"tw`from-emerald-50`"#####, r#####"({
  '--tw-gradient-from': "#ecfdf5 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(236 253 245 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "116")]
#[test_case(r#####"tw`from-emerald-100`"#####, r#####"({
  '--tw-gradient-from': "#d1fae5 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(209 250 229 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "117")]
#[test_case(r#####"tw`from-emerald-200`"#####, r#####"({
  '--tw-gradient-from': "#a7f3d0 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(167 243 208 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "118")]
#[test_case(r#####"tw`from-emerald-300`"#####, r#####"({
  '--tw-gradient-from': "#6ee7b7 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(110 231 183 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "119")]
#[test_case(r#####"tw`from-emerald-400`"#####, r#####"({
  '--tw-gradient-from': "#34d399 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(52 211 153 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "120")]
#[test_case(r#####"tw`from-emerald-500`"#####, r#####"({
  '--tw-gradient-from': "#10b981 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(16 185 129 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "121")]
#[test_case(r#####"tw`from-emerald-600`"#####, r#####"({
  '--tw-gradient-from': "#059669 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(5 150 105 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "122")]
#[test_case(r#####"tw`from-emerald-700`"#####, r#####"({
  '--tw-gradient-from': "#047857 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(4 120 87 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "123")]
#[test_case(r#####"tw`from-emerald-800`"#####, r#####"({
  '--tw-gradient-from': "#065f46 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(6 95 70 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "124")]
#[test_case(r#####"tw`from-emerald-900`"#####, r#####"({
  '--tw-gradient-from': "#064e3b var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(6 78 59 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "125")]
#[test_case(r#####"tw`from-teal-50`"#####, r#####"({
  '--tw-gradient-from': "#f0fdfa var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(240 253 250 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "126")]
#[test_case(r#####"tw`from-teal-100`"#####, r#####"({
  '--tw-gradient-from': "#ccfbf1 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(204 251 241 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "127")]
#[test_case(r#####"tw`from-teal-200`"#####, r#####"({
  '--tw-gradient-from': "#99f6e4 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(153 246 228 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "128")]
#[test_case(r#####"tw`from-teal-300`"#####, r#####"({
  '--tw-gradient-from': "#5eead4 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(94 234 212 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "129")]
#[test_case(r#####"tw`from-teal-400`"#####, r#####"({
  '--tw-gradient-from': "#2dd4bf var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(45 212 191 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "130")]
#[test_case(r#####"tw`from-teal-500`"#####, r#####"({
  '--tw-gradient-from': "#14b8a6 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(20 184 166 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "131")]
#[test_case(r#####"tw`from-teal-600`"#####, r#####"({
  '--tw-gradient-from': "#0d9488 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(13 148 136 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "132")]
#[test_case(r#####"tw`from-teal-700`"#####, r#####"({
  '--tw-gradient-from': "#0f766e var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(15 118 110 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "133")]
#[test_case(r#####"tw`from-teal-800`"#####, r#####"({
  '--tw-gradient-from': "#115e59 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(17 94 89 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "134")]
#[test_case(r#####"tw`from-teal-900`"#####, r#####"({
  '--tw-gradient-from': "#134e4a var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(19 78 74 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "135")]
#[test_case(r#####"tw`from-cyan-50`"#####, r#####"({
  '--tw-gradient-from': "#ecfeff var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(236 254 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "136")]
#[test_case(r#####"tw`from-cyan-100`"#####, r#####"({
  '--tw-gradient-from': "#cffafe var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(207 250 254 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "137")]
#[test_case(r#####"tw`from-cyan-200`"#####, r#####"({
  '--tw-gradient-from': "#a5f3fc var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(165 243 252 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "138")]
#[test_case(r#####"tw`from-cyan-300`"#####, r#####"({
  '--tw-gradient-from': "#67e8f9 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(103 232 249 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "139")]
#[test_case(r#####"tw`from-cyan-400`"#####, r#####"({
  '--tw-gradient-from': "#22d3ee var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(34 211 238 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "140")]
#[test_case(r#####"tw`from-cyan-500`"#####, r#####"({
  '--tw-gradient-from': "#06b6d4 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(6 182 212 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "141")]
#[test_case(r#####"tw`from-cyan-600`"#####, r#####"({
  '--tw-gradient-from': "#0891b2 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(8 145 178 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "142")]
#[test_case(r#####"tw`from-cyan-700`"#####, r#####"({
  '--tw-gradient-from': "#0e7490 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(14 116 144 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "143")]
#[test_case(r#####"tw`from-cyan-800`"#####, r#####"({
  '--tw-gradient-from': "#155e75 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(21 94 117 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "144")]
#[test_case(r#####"tw`from-cyan-900`"#####, r#####"({
  '--tw-gradient-from': "#164e63 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(22 78 99 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "145")]
#[test_case(r#####"tw`from-sky-50`"#####, r#####"({
  '--tw-gradient-from': "#f0f9ff var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(240 249 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "146")]
#[test_case(r#####"tw`from-sky-100`"#####, r#####"({
  '--tw-gradient-from': "#e0f2fe var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(224 242 254 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "147")]
#[test_case(r#####"tw`from-sky-200`"#####, r#####"({
  '--tw-gradient-from': "#bae6fd var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(186 230 253 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "148")]
#[test_case(r#####"tw`from-sky-300`"#####, r#####"({
  '--tw-gradient-from': "#7dd3fc var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(125 211 252 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "149")]
#[test_case(r#####"tw`from-sky-400`"#####, r#####"({
  '--tw-gradient-from': "#38bdf8 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(56 189 248 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "150")]
#[test_case(r#####"tw`from-sky-500`"#####, r#####"({
  '--tw-gradient-from': "#0ea5e9 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(14 165 233 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "151")]
#[test_case(r#####"tw`from-sky-600`"#####, r#####"({
  '--tw-gradient-from': "#0284c7 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(2 132 199 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "152")]
#[test_case(r#####"tw`from-sky-700`"#####, r#####"({
  '--tw-gradient-from': "#0369a1 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(3 105 161 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "153")]
#[test_case(r#####"tw`from-sky-800`"#####, r#####"({
  '--tw-gradient-from': "#075985 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(7 89 133 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "154")]
#[test_case(r#####"tw`from-sky-900`"#####, r#####"({
  '--tw-gradient-from': "#0c4a6e var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(12 74 110 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "155")]
#[test_case(r#####"tw`from-blue-50`"#####, r#####"({
  '--tw-gradient-from': "#eff6ff var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 246 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "156")]
#[test_case(r#####"tw`from-blue-100`"#####, r#####"({
  '--tw-gradient-from': "#dbeafe var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(219 234 254 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "157")]
#[test_case(r#####"tw`from-blue-200`"#####, r#####"({
  '--tw-gradient-from': "#bfdbfe var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(191 219 254 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "158")]
#[test_case(r#####"tw`from-blue-300`"#####, r#####"({
  '--tw-gradient-from': "#93c5fd var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(147 197 253 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "159")]
#[test_case(r#####"tw`from-blue-400`"#####, r#####"({
  '--tw-gradient-from': "#60a5fa var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(96 165 250 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "160")]
#[test_case(r#####"tw`from-blue-500`"#####, r#####"({
  '--tw-gradient-from': "#3b82f6 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(59 130 246 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "161")]
#[test_case(r#####"tw`from-blue-600`"#####, r#####"({
  '--tw-gradient-from': "#2563eb var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(37 99 235 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "162")]
#[test_case(r#####"tw`from-blue-700`"#####, r#####"({
  '--tw-gradient-from': "#1d4ed8 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(29 78 216 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "163")]
#[test_case(r#####"tw`from-blue-800`"#####, r#####"({
  '--tw-gradient-from': "#1e40af var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(30 64 175 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "164")]
#[test_case(r#####"tw`from-blue-900`"#####, r#####"({
  '--tw-gradient-from': "#1e3a8a var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(30 58 138 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "165")]
#[test_case(r#####"tw`from-indigo-50`"#####, r#####"({
  '--tw-gradient-from': "#eef2ff var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(238 242 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "166")]
#[test_case(r#####"tw`from-indigo-100`"#####, r#####"({
  '--tw-gradient-from': "#e0e7ff var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(224 231 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "167")]
#[test_case(r#####"tw`from-indigo-200`"#####, r#####"({
  '--tw-gradient-from': "#c7d2fe var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(199 210 254 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "168")]
#[test_case(r#####"tw`from-indigo-300`"#####, r#####"({
  '--tw-gradient-from': "#a5b4fc var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(165 180 252 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "169")]
#[test_case(r#####"tw`from-indigo-400`"#####, r#####"({
  '--tw-gradient-from': "#818cf8 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(129 140 248 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "170")]
#[test_case(r#####"tw`from-indigo-500`"#####, r#####"({
  '--tw-gradient-from': "#6366f1 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(99 102 241 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "171")]
#[test_case(r#####"tw`from-indigo-600`"#####, r#####"({
  '--tw-gradient-from': "#4f46e5 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(79 70 229 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "172")]
#[test_case(r#####"tw`from-indigo-700`"#####, r#####"({
  '--tw-gradient-from': "#4338ca var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(67 56 202 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "173")]
#[test_case(r#####"tw`from-indigo-800`"#####, r#####"({
  '--tw-gradient-from': "#3730a3 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(55 48 163 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "174")]
#[test_case(r#####"tw`from-indigo-900`"#####, r#####"({
  '--tw-gradient-from': "#312e81 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(49 46 129 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "175")]
#[test_case(r#####"tw`from-violet-50`"#####, r#####"({
  '--tw-gradient-from': "#f5f3ff var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(245 243 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "176")]
#[test_case(r#####"tw`from-violet-100`"#####, r#####"({
  '--tw-gradient-from': "#ede9fe var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(237 233 254 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "177")]
#[test_case(r#####"tw`from-violet-200`"#####, r#####"({
  '--tw-gradient-from': "#ddd6fe var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(221 214 254 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "178")]
#[test_case(r#####"tw`from-violet-300`"#####, r#####"({
  '--tw-gradient-from': "#c4b5fd var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(196 181 253 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "179")]
#[test_case(r#####"tw`from-violet-400`"#####, r#####"({
  '--tw-gradient-from': "#a78bfa var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(167 139 250 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "180")]
#[test_case(r#####"tw`from-violet-500`"#####, r#####"({
  '--tw-gradient-from': "#8b5cf6 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(139 92 246 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "181")]
#[test_case(r#####"tw`from-violet-600`"#####, r#####"({
  '--tw-gradient-from': "#7c3aed var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(124 58 237 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "182")]
#[test_case(r#####"tw`from-violet-700`"#####, r#####"({
  '--tw-gradient-from': "#6d28d9 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(109 40 217 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "183")]
#[test_case(r#####"tw`from-violet-800`"#####, r#####"({
  '--tw-gradient-from': "#5b21b6 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(91 33 182 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "184")]
#[test_case(r#####"tw`from-violet-900`"#####, r#####"({
  '--tw-gradient-from': "#4c1d95 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(76 29 149 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "185")]
#[test_case(r#####"tw`from-purple-50`"#####, r#####"({
  '--tw-gradient-from': "#faf5ff var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(250 245 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "186")]
#[test_case(r#####"tw`from-purple-100`"#####, r#####"({
  '--tw-gradient-from': "#f3e8ff var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(243 232 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "187")]
#[test_case(r#####"tw`from-purple-200`"#####, r#####"({
  '--tw-gradient-from': "#e9d5ff var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(233 213 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "188")]
#[test_case(r#####"tw`from-purple-300`"#####, r#####"({
  '--tw-gradient-from': "#d8b4fe var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(216 180 254 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "189")]
#[test_case(r#####"tw`from-purple-400`"#####, r#####"({
  '--tw-gradient-from': "#c084fc var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(192 132 252 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "190")]
#[test_case(r#####"tw`from-purple-500`"#####, r#####"({
  '--tw-gradient-from': "#a855f7 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(168 85 247 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "191")]
#[test_case(r#####"tw`from-purple-600`"#####, r#####"({
  '--tw-gradient-from': "#9333ea var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(147 51 234 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "192")]
#[test_case(r#####"tw`from-purple-700`"#####, r#####"({
  '--tw-gradient-from': "#7e22ce var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(126 34 206 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "193")]
#[test_case(r#####"tw`from-purple-800`"#####, r#####"({
  '--tw-gradient-from': "#6b21a8 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(107 33 168 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "194")]
#[test_case(r#####"tw`from-purple-900`"#####, r#####"({
  '--tw-gradient-from': "#581c87 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(88 28 135 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "195")]
#[test_case(r#####"tw`from-fuchsia-50`"#####, r#####"({
  '--tw-gradient-from': "#fdf4ff var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(253 244 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "196")]
#[test_case(r#####"tw`from-fuchsia-100`"#####, r#####"({
  '--tw-gradient-from': "#fae8ff var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(250 232 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "197")]
#[test_case(r#####"tw`from-fuchsia-200`"#####, r#####"({
  '--tw-gradient-from': "#f5d0fe var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(245 208 254 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "198")]
#[test_case(r#####"tw`from-fuchsia-300`"#####, r#####"({
  '--tw-gradient-from': "#f0abfc var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(240 171 252 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "199")]
#[test_case(r#####"tw`from-fuchsia-400`"#####, r#####"({
  '--tw-gradient-from': "#e879f9 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(232 121 249 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "200")]
#[test_case(r#####"tw`from-fuchsia-500`"#####, r#####"({
  '--tw-gradient-from': "#d946ef var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(217 70 239 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "201")]
#[test_case(r#####"tw`from-fuchsia-600`"#####, r#####"({
  '--tw-gradient-from': "#c026d3 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(192 38 211 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "202")]
#[test_case(r#####"tw`from-fuchsia-700`"#####, r#####"({
  '--tw-gradient-from': "#a21caf var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(162 28 175 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "203")]
#[test_case(r#####"tw`from-fuchsia-800`"#####, r#####"({
  '--tw-gradient-from': "#86198f var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(134 25 143 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "204")]
#[test_case(r#####"tw`from-fuchsia-900`"#####, r#####"({
  '--tw-gradient-from': "#701a75 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(112 26 117 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "205")]
#[test_case(r#####"tw`from-pink-50`"#####, r#####"({
  '--tw-gradient-from': "#fdf2f8 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(253 242 248 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "206")]
#[test_case(r#####"tw`from-pink-100`"#####, r#####"({
  '--tw-gradient-from': "#fce7f3 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(252 231 243 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "207")]
#[test_case(r#####"tw`from-pink-200`"#####, r#####"({
  '--tw-gradient-from': "#fbcfe8 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(251 207 232 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "208")]
#[test_case(r#####"tw`from-pink-300`"#####, r#####"({
  '--tw-gradient-from': "#f9a8d4 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(249 168 212 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "209")]
#[test_case(r#####"tw`from-pink-400`"#####, r#####"({
  '--tw-gradient-from': "#f472b6 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(244 114 182 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "210")]
#[test_case(r#####"tw`from-pink-500`"#####, r#####"({
  '--tw-gradient-from': "#ec4899 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(236 72 153 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "211")]
#[test_case(r#####"tw`from-pink-600`"#####, r#####"({
  '--tw-gradient-from': "#db2777 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(219 39 119 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "212")]
#[test_case(r#####"tw`from-pink-700`"#####, r#####"({
  '--tw-gradient-from': "#be185d var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(190 24 93 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "213")]
#[test_case(r#####"tw`from-pink-800`"#####, r#####"({
  '--tw-gradient-from': "#9d174d var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(157 23 77 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "214")]
#[test_case(r#####"tw`from-pink-900`"#####, r#####"({
  '--tw-gradient-from': "#831843 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(131 24 67 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "215")]
#[test_case(r#####"tw`from-rose-50`"#####, r#####"({
  '--tw-gradient-from': "#fff1f2 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 241 242 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "216")]
#[test_case(r#####"tw`from-rose-100`"#####, r#####"({
  '--tw-gradient-from': "#ffe4e6 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 228 230 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "217")]
#[test_case(r#####"tw`from-rose-200`"#####, r#####"({
  '--tw-gradient-from': "#fecdd3 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 205 211 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "218")]
#[test_case(r#####"tw`from-rose-300`"#####, r#####"({
  '--tw-gradient-from': "#fda4af var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(253 164 175 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "219")]
#[test_case(r#####"tw`from-rose-400`"#####, r#####"({
  '--tw-gradient-from': "#fb7185 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(251 113 133 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "220")]
#[test_case(r#####"tw`from-rose-500`"#####, r#####"({
  '--tw-gradient-from': "#f43f5e var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(244 63 94 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "221")]
#[test_case(r#####"tw`from-rose-600`"#####, r#####"({
  '--tw-gradient-from': "#e11d48 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(225 29 72 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "222")]
#[test_case(r#####"tw`from-rose-700`"#####, r#####"({
  '--tw-gradient-from': "#be123c var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(190 18 60 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "223")]
#[test_case(r#####"tw`from-rose-800`"#####, r#####"({
  '--tw-gradient-from': "#9f1239 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(159 18 57 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "224")]
#[test_case(r#####"tw`from-rose-900`"#####, r#####"({
  '--tw-gradient-from': "#881337 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(136 19 55 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "225")]
#[test_case(r#####"tw`via-inherit`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 255 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), inherit var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "226")]
#[test_case(r#####"tw`via-current`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 255 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), currentColor var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "227")]
#[test_case(r#####"tw`via-transparent`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(0 0 0 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), transparent var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "228")]
#[test_case(r#####"tw`via-black`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(0 0 0 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #000 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "229")]
#[test_case(r#####"tw`via-white`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 255 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fff var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "230")]
#[test_case(r#####"tw`via-slate-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(248 250 252 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f8fafc var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "231")]
#[test_case(r#####"tw`via-slate-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(241 245 249 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f1f5f9 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "232")]
#[test_case(r#####"tw`via-slate-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(226 232 240 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #e2e8f0 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "233")]
#[test_case(r#####"tw`via-slate-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(203 213 225 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #cbd5e1 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "234")]
#[test_case(r#####"tw`via-slate-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(148 163 184 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #94a3b8 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "235")]
#[test_case(r#####"tw`via-slate-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(100 116 139 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #64748b var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "236")]
#[test_case(r#####"tw`via-slate-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(71 85 105 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #475569 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "237")]
#[test_case(r#####"tw`via-slate-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(51 65 85 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #334155 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "238")]
#[test_case(r#####"tw`via-slate-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(30 41 59 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #1e293b var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "239")]
#[test_case(r#####"tw`via-slate-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(15 23 42 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #0f172a var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "240")]
#[test_case(r#####"tw`via-gray-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(249 250 251 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f9fafb var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "241")]
#[test_case(r#####"tw`via-gray-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(243 244 246 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f3f4f6 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "242")]
#[test_case(r#####"tw`via-gray-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(229 231 235 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #e5e7eb var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "243")]
#[test_case(r#####"tw`via-gray-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(209 213 219 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #d1d5db var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "244")]
#[test_case(r#####"tw`via-gray-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(156 163 175 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #9ca3af var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "245")]
#[test_case(r#####"tw`via-gray-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(107 114 128 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #6b7280 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "246")]
#[test_case(r#####"tw`via-gray-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(75 85 99 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #4b5563 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "247")]
#[test_case(r#####"tw`via-gray-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(55 65 81 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #374151 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "248")]
#[test_case(r#####"tw`via-gray-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(31 41 55 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #1f2937 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "249")]
#[test_case(r#####"tw`via-gray-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(17 24 39 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #111827 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "250")]
#[test_case(r#####"tw`via-zinc-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(250 250 250 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fafafa var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "251")]
#[test_case(r#####"tw`via-zinc-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(244 244 245 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f4f4f5 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "252")]
#[test_case(r#####"tw`via-zinc-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(228 228 231 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #e4e4e7 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "253")]
#[test_case(r#####"tw`via-zinc-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(212 212 216 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #d4d4d8 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "254")]
#[test_case(r#####"tw`via-zinc-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(161 161 170 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #a1a1aa var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "255")]
#[test_case(r#####"tw`via-zinc-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(113 113 122 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #71717a var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "256")]
#[test_case(r#####"tw`via-zinc-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(82 82 91 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #52525b var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "257")]
#[test_case(r#####"tw`via-zinc-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(63 63 70 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #3f3f46 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "258")]
#[test_case(r#####"tw`via-zinc-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(39 39 42 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #27272a var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "259")]
#[test_case(r#####"tw`via-zinc-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(24 24 27 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #18181b var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "260")]
#[test_case(r#####"tw`via-neutral-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(250 250 250 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fafafa var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "261")]
#[test_case(r#####"tw`via-neutral-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(245 245 245 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f5f5f5 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "262")]
#[test_case(r#####"tw`via-neutral-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(229 229 229 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #e5e5e5 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "263")]
#[test_case(r#####"tw`via-neutral-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(212 212 212 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #d4d4d4 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "264")]
#[test_case(r#####"tw`via-neutral-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(163 163 163 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #a3a3a3 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "265")]
#[test_case(r#####"tw`via-neutral-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(115 115 115 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #737373 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "266")]
#[test_case(r#####"tw`via-neutral-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(82 82 82 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #525252 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "267")]
#[test_case(r#####"tw`via-neutral-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(64 64 64 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #404040 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "268")]
#[test_case(r#####"tw`via-neutral-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(38 38 38 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #262626 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "269")]
#[test_case(r#####"tw`via-neutral-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(23 23 23 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #171717 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "270")]
#[test_case(r#####"tw`via-stone-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(250 250 249 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fafaf9 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "271")]
#[test_case(r#####"tw`via-stone-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(245 245 244 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f5f5f4 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "272")]
#[test_case(r#####"tw`via-stone-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(231 229 228 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #e7e5e4 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "273")]
#[test_case(r#####"tw`via-stone-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(214 211 209 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #d6d3d1 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "274")]
#[test_case(r#####"tw`via-stone-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(168 162 158 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #a8a29e var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "275")]
#[test_case(r#####"tw`via-stone-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(120 113 108 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #78716c var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "276")]
#[test_case(r#####"tw`via-stone-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(87 83 78 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #57534e var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "277")]
#[test_case(r#####"tw`via-stone-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(68 64 60 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #44403c var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "278")]
#[test_case(r#####"tw`via-stone-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(41 37 36 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #292524 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "279")]
#[test_case(r#####"tw`via-stone-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(28 25 23 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #1c1917 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "280")]
#[test_case(r#####"tw`via-red-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 242 242 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fef2f2 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "281")]
#[test_case(r#####"tw`via-red-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 226 226 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fee2e2 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "282")]
#[test_case(r#####"tw`via-red-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 202 202 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fecaca var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "283")]
#[test_case(r#####"tw`via-red-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(252 165 165 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fca5a5 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "284")]
#[test_case(r#####"tw`via-red-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(248 113 113 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f87171 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "285")]
#[test_case(r#####"tw`via-red-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 68 68 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ef4444 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "286")]
#[test_case(r#####"tw`via-red-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(220 38 38 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #dc2626 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "287")]
#[test_case(r#####"tw`via-red-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(185 28 28 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #b91c1c var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "288")]
#[test_case(r#####"tw`via-red-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(153 27 27 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #991b1b var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "289")]
#[test_case(r#####"tw`via-red-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(127 29 29 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #7f1d1d var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "290")]
#[test_case(r#####"tw`via-orange-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 247 237 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fff7ed var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "291")]
#[test_case(r#####"tw`via-orange-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 237 213 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ffedd5 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "292")]
#[test_case(r#####"tw`via-orange-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 215 170 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fed7aa var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "293")]
#[test_case(r#####"tw`via-orange-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(253 186 116 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fdba74 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "294")]
#[test_case(r#####"tw`via-orange-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(251 146 60 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fb923c var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "295")]
#[test_case(r#####"tw`via-orange-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(249 115 22 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f97316 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "296")]
#[test_case(r#####"tw`via-orange-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(234 88 12 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ea580c var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "297")]
#[test_case(r#####"tw`via-orange-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(194 65 12 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #c2410c var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "298")]
#[test_case(r#####"tw`via-orange-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(154 52 18 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #9a3412 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "299")]
#[test_case(r#####"tw`via-orange-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(124 45 18 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #7c2d12 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "300")]
#[test_case(r#####"tw`via-amber-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 251 235 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fffbeb var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "301")]
#[test_case(r#####"tw`via-amber-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 243 199 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fef3c7 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "302")]
#[test_case(r#####"tw`via-amber-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(253 230 138 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fde68a var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "303")]
#[test_case(r#####"tw`via-amber-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(252 211 77 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fcd34d var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "304")]
#[test_case(r#####"tw`via-amber-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(251 191 36 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fbbf24 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "305")]
#[test_case(r#####"tw`via-amber-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(245 158 11 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f59e0b var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "306")]
#[test_case(r#####"tw`via-amber-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(217 119 6 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #d97706 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "307")]
#[test_case(r#####"tw`via-amber-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(180 83 9 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #b45309 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "308")]
#[test_case(r#####"tw`via-amber-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(146 64 14 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #92400e var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "309")]
#[test_case(r#####"tw`via-amber-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(120 53 15 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #78350f var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "310")]
#[test_case(r#####"tw`via-yellow-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 252 232 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fefce8 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "311")]
#[test_case(r#####"tw`via-yellow-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 249 195 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fef9c3 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "312")]
#[test_case(r#####"tw`via-yellow-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 240 138 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fef08a var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "313")]
#[test_case(r#####"tw`via-yellow-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(253 224 71 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fde047 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "314")]
#[test_case(r#####"tw`via-yellow-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(250 204 21 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #facc15 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "315")]
#[test_case(r#####"tw`via-yellow-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(234 179 8 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #eab308 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "316")]
#[test_case(r#####"tw`via-yellow-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(202 138 4 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ca8a04 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "317")]
#[test_case(r#####"tw`via-yellow-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(161 98 7 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #a16207 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "318")]
#[test_case(r#####"tw`via-yellow-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(133 77 14 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #854d0e var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "319")]
#[test_case(r#####"tw`via-yellow-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(113 63 18 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #713f12 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "320")]
#[test_case(r#####"tw`via-lime-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(247 254 231 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f7fee7 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "321")]
#[test_case(r#####"tw`via-lime-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(236 252 203 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ecfccb var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "322")]
#[test_case(r#####"tw`via-lime-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(217 249 157 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #d9f99d var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "323")]
#[test_case(r#####"tw`via-lime-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(190 242 100 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #bef264 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "324")]
#[test_case(r#####"tw`via-lime-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(163 230 53 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #a3e635 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "325")]
#[test_case(r#####"tw`via-lime-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(132 204 22 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #84cc16 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "326")]
#[test_case(r#####"tw`via-lime-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(101 163 13 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #65a30d var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "327")]
#[test_case(r#####"tw`via-lime-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(77 124 15 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #4d7c0f var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "328")]
#[test_case(r#####"tw`via-lime-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(63 98 18 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #3f6212 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "329")]
#[test_case(r#####"tw`via-lime-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(54 83 20 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #365314 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "330")]
#[test_case(r#####"tw`via-green-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(240 253 244 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f0fdf4 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "331")]
#[test_case(r#####"tw`via-green-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(220 252 231 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #dcfce7 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "332")]
#[test_case(r#####"tw`via-green-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(187 247 208 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #bbf7d0 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "333")]
#[test_case(r#####"tw`via-green-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(134 239 172 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #86efac var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "334")]
#[test_case(r#####"tw`via-green-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(74 222 128 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #4ade80 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "335")]
#[test_case(r#####"tw`via-green-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(34 197 94 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #22c55e var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "336")]
#[test_case(r#####"tw`via-green-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(22 163 74 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #16a34a var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "337")]
#[test_case(r#####"tw`via-green-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(21 128 61 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #15803d var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "338")]
#[test_case(r#####"tw`via-green-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(22 101 52 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #166534 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "339")]
#[test_case(r#####"tw`via-green-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(20 83 45 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #14532d var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "340")]
#[test_case(r#####"tw`via-emerald-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(236 253 245 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ecfdf5 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "341")]
#[test_case(r#####"tw`via-emerald-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(209 250 229 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #d1fae5 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "342")]
#[test_case(r#####"tw`via-emerald-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(167 243 208 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #a7f3d0 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "343")]
#[test_case(r#####"tw`via-emerald-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(110 231 183 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #6ee7b7 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "344")]
#[test_case(r#####"tw`via-emerald-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(52 211 153 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #34d399 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "345")]
#[test_case(r#####"tw`via-emerald-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(16 185 129 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #10b981 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "346")]
#[test_case(r#####"tw`via-emerald-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(5 150 105 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #059669 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "347")]
#[test_case(r#####"tw`via-emerald-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(4 120 87 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #047857 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "348")]
#[test_case(r#####"tw`via-emerald-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(6 95 70 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #065f46 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "349")]
#[test_case(r#####"tw`via-emerald-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(6 78 59 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #064e3b var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "350")]
#[test_case(r#####"tw`via-teal-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(240 253 250 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f0fdfa var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "351")]
#[test_case(r#####"tw`via-teal-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(204 251 241 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ccfbf1 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "352")]
#[test_case(r#####"tw`via-teal-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(153 246 228 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #99f6e4 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "353")]
#[test_case(r#####"tw`via-teal-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(94 234 212 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #5eead4 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "354")]
#[test_case(r#####"tw`via-teal-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(45 212 191 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #2dd4bf var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "355")]
#[test_case(r#####"tw`via-teal-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(20 184 166 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #14b8a6 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "356")]
#[test_case(r#####"tw`via-teal-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(13 148 136 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #0d9488 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "357")]
#[test_case(r#####"tw`via-teal-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(15 118 110 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #0f766e var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "358")]
#[test_case(r#####"tw`via-teal-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(17 94 89 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #115e59 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "359")]
#[test_case(r#####"tw`via-teal-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(19 78 74 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #134e4a var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "360")]
#[test_case(r#####"tw`via-cyan-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(236 254 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ecfeff var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "361")]
#[test_case(r#####"tw`via-cyan-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(207 250 254 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #cffafe var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "362")]
#[test_case(r#####"tw`via-cyan-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(165 243 252 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #a5f3fc var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "363")]
#[test_case(r#####"tw`via-cyan-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(103 232 249 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #67e8f9 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "364")]
#[test_case(r#####"tw`via-cyan-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(34 211 238 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #22d3ee var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "365")]
#[test_case(r#####"tw`via-cyan-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(6 182 212 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #06b6d4 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "366")]
#[test_case(r#####"tw`via-cyan-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(8 145 178 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #0891b2 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "367")]
#[test_case(r#####"tw`via-cyan-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(14 116 144 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #0e7490 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "368")]
#[test_case(r#####"tw`via-cyan-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(21 94 117 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #155e75 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "369")]
#[test_case(r#####"tw`via-cyan-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(22 78 99 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #164e63 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "370")]
#[test_case(r#####"tw`via-sky-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(240 249 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f0f9ff var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "371")]
#[test_case(r#####"tw`via-sky-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(224 242 254 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #e0f2fe var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "372")]
#[test_case(r#####"tw`via-sky-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(186 230 253 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #bae6fd var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "373")]
#[test_case(r#####"tw`via-sky-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(125 211 252 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #7dd3fc var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "374")]
#[test_case(r#####"tw`via-sky-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(56 189 248 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #38bdf8 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "375")]
#[test_case(r#####"tw`via-sky-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(14 165 233 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #0ea5e9 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "376")]
#[test_case(r#####"tw`via-sky-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(2 132 199 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #0284c7 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "377")]
#[test_case(r#####"tw`via-sky-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(3 105 161 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #0369a1 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "378")]
#[test_case(r#####"tw`via-sky-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(7 89 133 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #075985 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "379")]
#[test_case(r#####"tw`via-sky-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(12 74 110 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #0c4a6e var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "380")]
#[test_case(r#####"tw`via-blue-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 246 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #eff6ff var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "381")]
#[test_case(r#####"tw`via-blue-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(219 234 254 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #dbeafe var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "382")]
#[test_case(r#####"tw`via-blue-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(191 219 254 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #bfdbfe var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "383")]
#[test_case(r#####"tw`via-blue-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(147 197 253 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #93c5fd var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "384")]
#[test_case(r#####"tw`via-blue-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(96 165 250 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #60a5fa var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "385")]
#[test_case(r#####"tw`via-blue-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(59 130 246 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #3b82f6 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "386")]
#[test_case(r#####"tw`via-blue-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(37 99 235 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #2563eb var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "387")]
#[test_case(r#####"tw`via-blue-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(29 78 216 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #1d4ed8 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "388")]
#[test_case(r#####"tw`via-blue-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(30 64 175 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #1e40af var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "389")]
#[test_case(r#####"tw`via-blue-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(30 58 138 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #1e3a8a var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "390")]
#[test_case(r#####"tw`via-indigo-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(238 242 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #eef2ff var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "391")]
#[test_case(r#####"tw`via-indigo-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(224 231 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #e0e7ff var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "392")]
#[test_case(r#####"tw`via-indigo-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(199 210 254 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #c7d2fe var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "393")]
#[test_case(r#####"tw`via-indigo-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(165 180 252 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #a5b4fc var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "394")]
#[test_case(r#####"tw`via-indigo-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(129 140 248 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #818cf8 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "395")]
#[test_case(r#####"tw`via-indigo-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(99 102 241 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #6366f1 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "396")]
#[test_case(r#####"tw`via-indigo-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(79 70 229 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #4f46e5 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "397")]
#[test_case(r#####"tw`via-indigo-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(67 56 202 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #4338ca var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "398")]
#[test_case(r#####"tw`via-indigo-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(55 48 163 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #3730a3 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "399")]
#[test_case(r#####"tw`via-indigo-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(49 46 129 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #312e81 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "400")]
#[test_case(r#####"tw`via-violet-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(245 243 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f5f3ff var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "401")]
#[test_case(r#####"tw`via-violet-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(237 233 254 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ede9fe var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "402")]
#[test_case(r#####"tw`via-violet-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(221 214 254 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ddd6fe var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "403")]
#[test_case(r#####"tw`via-violet-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(196 181 253 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #c4b5fd var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "404")]
#[test_case(r#####"tw`via-violet-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(167 139 250 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #a78bfa var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "405")]
#[test_case(r#####"tw`via-violet-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(139 92 246 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #8b5cf6 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "406")]
#[test_case(r#####"tw`via-violet-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(124 58 237 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #7c3aed var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "407")]
#[test_case(r#####"tw`via-violet-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(109 40 217 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #6d28d9 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "408")]
#[test_case(r#####"tw`via-violet-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(91 33 182 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #5b21b6 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "409")]
#[test_case(r#####"tw`via-violet-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(76 29 149 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #4c1d95 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "410")]
#[test_case(r#####"tw`via-purple-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(250 245 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #faf5ff var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "411")]
#[test_case(r#####"tw`via-purple-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(243 232 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f3e8ff var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "412")]
#[test_case(r#####"tw`via-purple-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(233 213 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #e9d5ff var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "413")]
#[test_case(r#####"tw`via-purple-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(216 180 254 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #d8b4fe var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "414")]
#[test_case(r#####"tw`via-purple-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(192 132 252 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #c084fc var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "415")]
#[test_case(r#####"tw`via-purple-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(168 85 247 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #a855f7 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "416")]
#[test_case(r#####"tw`via-purple-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(147 51 234 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #9333ea var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "417")]
#[test_case(r#####"tw`via-purple-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(126 34 206 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #7e22ce var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "418")]
#[test_case(r#####"tw`via-purple-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(107 33 168 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #6b21a8 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "419")]
#[test_case(r#####"tw`via-purple-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(88 28 135 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #581c87 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "420")]
#[test_case(r#####"tw`via-fuchsia-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(253 244 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fdf4ff var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "421")]
#[test_case(r#####"tw`via-fuchsia-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(250 232 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fae8ff var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "422")]
#[test_case(r#####"tw`via-fuchsia-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(245 208 254 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f5d0fe var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "423")]
#[test_case(r#####"tw`via-fuchsia-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(240 171 252 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f0abfc var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "424")]
#[test_case(r#####"tw`via-fuchsia-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(232 121 249 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #e879f9 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "425")]
#[test_case(r#####"tw`via-fuchsia-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(217 70 239 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #d946ef var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "426")]
#[test_case(r#####"tw`via-fuchsia-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(192 38 211 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #c026d3 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "427")]
#[test_case(r#####"tw`via-fuchsia-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(162 28 175 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #a21caf var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "428")]
#[test_case(r#####"tw`via-fuchsia-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(134 25 143 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #86198f var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "429")]
#[test_case(r#####"tw`via-fuchsia-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(112 26 117 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #701a75 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "430")]
#[test_case(r#####"tw`via-pink-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(253 242 248 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fdf2f8 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "431")]
#[test_case(r#####"tw`via-pink-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(252 231 243 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fce7f3 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "432")]
#[test_case(r#####"tw`via-pink-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(251 207 232 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fbcfe8 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "433")]
#[test_case(r#####"tw`via-pink-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(249 168 212 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f9a8d4 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "434")]
#[test_case(r#####"tw`via-pink-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(244 114 182 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f472b6 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "435")]
#[test_case(r#####"tw`via-pink-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(236 72 153 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ec4899 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "436")]
#[test_case(r#####"tw`via-pink-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(219 39 119 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #db2777 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "437")]
#[test_case(r#####"tw`via-pink-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(190 24 93 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #be185d var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "438")]
#[test_case(r#####"tw`via-pink-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(157 23 77 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #9d174d var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "439")]
#[test_case(r#####"tw`via-pink-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(131 24 67 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #831843 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "440")]
#[test_case(r#####"tw`via-rose-50`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 241 242 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fff1f2 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "441")]
#[test_case(r#####"tw`via-rose-100`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(255 228 230 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ffe4e6 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "442")]
#[test_case(r#####"tw`via-rose-200`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(254 205 211 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fecdd3 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "443")]
#[test_case(r#####"tw`via-rose-300`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(253 164 175 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fda4af var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "444")]
#[test_case(r#####"tw`via-rose-400`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(251 113 133 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #fb7185 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "445")]
#[test_case(r#####"tw`via-rose-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(244 63 94 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #f43f5e var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "446")]
#[test_case(r#####"tw`via-rose-600`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(225 29 72 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #e11d48 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "447")]
#[test_case(r#####"tw`via-rose-700`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(190 18 60 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #be123c var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "448")]
#[test_case(r#####"tw`via-rose-800`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(159 18 57 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #9f1239 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "449")]
#[test_case(r#####"tw`via-rose-900`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(136 19 55 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #881337 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "450")]
#[test_case(r#####"tw`to-inherit`"#####, r#####"({
  '--tw-gradient-to': "inherit var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "451")]
#[test_case(r#####"tw`to-current`"#####, r#####"({
  '--tw-gradient-to': "currentColor var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "452")]
#[test_case(r#####"tw`to-transparent`"#####, r#####"({
  '--tw-gradient-to': "transparent var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "453")]
#[test_case(r#####"tw`to-black`"#####, r#####"({
  '--tw-gradient-to': "#000 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "454")]
#[test_case(r#####"tw`to-white`"#####, r#####"({
  '--tw-gradient-to': "#fff var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "455")]
#[test_case(r#####"tw`to-slate-50`"#####, r#####"({
  '--tw-gradient-to': "#f8fafc var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "456")]
#[test_case(r#####"tw`to-slate-100`"#####, r#####"({
  '--tw-gradient-to': "#f1f5f9 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "457")]
#[test_case(r#####"tw`to-slate-200`"#####, r#####"({
  '--tw-gradient-to': "#e2e8f0 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "458")]
#[test_case(r#####"tw`to-slate-300`"#####, r#####"({
  '--tw-gradient-to': "#cbd5e1 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "459")]
#[test_case(r#####"tw`to-slate-400`"#####, r#####"({
  '--tw-gradient-to': "#94a3b8 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "460")]
#[test_case(r#####"tw`to-slate-500`"#####, r#####"({
  '--tw-gradient-to': "#64748b var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "461")]
#[test_case(r#####"tw`to-slate-600`"#####, r#####"({
  '--tw-gradient-to': "#475569 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "462")]
#[test_case(r#####"tw`to-slate-700`"#####, r#####"({
  '--tw-gradient-to': "#334155 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "463")]
#[test_case(r#####"tw`to-slate-800`"#####, r#####"({
  '--tw-gradient-to': "#1e293b var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "464")]
#[test_case(r#####"tw`to-slate-900`"#####, r#####"({
  '--tw-gradient-to': "#0f172a var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "465")]
#[test_case(r#####"tw`to-gray-50`"#####, r#####"({
  '--tw-gradient-to': "#f9fafb var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "466")]
#[test_case(r#####"tw`to-gray-100`"#####, r#####"({
  '--tw-gradient-to': "#f3f4f6 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "467")]
#[test_case(r#####"tw`to-gray-200`"#####, r#####"({
  '--tw-gradient-to': "#e5e7eb var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "468")]
#[test_case(r#####"tw`to-gray-300`"#####, r#####"({
  '--tw-gradient-to': "#d1d5db var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "469")]
#[test_case(r#####"tw`to-gray-400`"#####, r#####"({
  '--tw-gradient-to': "#9ca3af var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "470")]
#[test_case(r#####"tw`to-gray-500`"#####, r#####"({
  '--tw-gradient-to': "#6b7280 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "471")]
#[test_case(r#####"tw`to-gray-600`"#####, r#####"({
  '--tw-gradient-to': "#4b5563 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "472")]
#[test_case(r#####"tw`to-gray-700`"#####, r#####"({
  '--tw-gradient-to': "#374151 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "473")]
#[test_case(r#####"tw`to-gray-800`"#####, r#####"({
  '--tw-gradient-to': "#1f2937 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "474")]
#[test_case(r#####"tw`to-gray-900`"#####, r#####"({
  '--tw-gradient-to': "#111827 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "475")]
#[test_case(r#####"tw`to-zinc-50`"#####, r#####"({
  '--tw-gradient-to': "#fafafa var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "476")]
#[test_case(r#####"tw`to-zinc-100`"#####, r#####"({
  '--tw-gradient-to': "#f4f4f5 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "477")]
#[test_case(r#####"tw`to-zinc-200`"#####, r#####"({
  '--tw-gradient-to': "#e4e4e7 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "478")]
#[test_case(r#####"tw`to-zinc-300`"#####, r#####"({
  '--tw-gradient-to': "#d4d4d8 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "479")]
#[test_case(r#####"tw`to-zinc-400`"#####, r#####"({
  '--tw-gradient-to': "#a1a1aa var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "480")]
#[test_case(r#####"tw`to-zinc-500`"#####, r#####"({
  '--tw-gradient-to': "#71717a var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "481")]
#[test_case(r#####"tw`to-zinc-600`"#####, r#####"({
  '--tw-gradient-to': "#52525b var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "482")]
#[test_case(r#####"tw`to-zinc-700`"#####, r#####"({
  '--tw-gradient-to': "#3f3f46 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "483")]
#[test_case(r#####"tw`to-zinc-800`"#####, r#####"({
  '--tw-gradient-to': "#27272a var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "484")]
#[test_case(r#####"tw`to-zinc-900`"#####, r#####"({
  '--tw-gradient-to': "#18181b var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "485")]
#[test_case(r#####"tw`to-neutral-50`"#####, r#####"({
  '--tw-gradient-to': "#fafafa var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "486")]
#[test_case(r#####"tw`to-neutral-100`"#####, r#####"({
  '--tw-gradient-to': "#f5f5f5 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "487")]
#[test_case(r#####"tw`to-neutral-200`"#####, r#####"({
  '--tw-gradient-to': "#e5e5e5 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "488")]
#[test_case(r#####"tw`to-neutral-300`"#####, r#####"({
  '--tw-gradient-to': "#d4d4d4 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "489")]
#[test_case(r#####"tw`to-neutral-400`"#####, r#####"({
  '--tw-gradient-to': "#a3a3a3 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "490")]
#[test_case(r#####"tw`to-neutral-500`"#####, r#####"({
  '--tw-gradient-to': "#737373 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "491")]
#[test_case(r#####"tw`to-neutral-600`"#####, r#####"({
  '--tw-gradient-to': "#525252 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "492")]
#[test_case(r#####"tw`to-neutral-700`"#####, r#####"({
  '--tw-gradient-to': "#404040 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "493")]
#[test_case(r#####"tw`to-neutral-800`"#####, r#####"({
  '--tw-gradient-to': "#262626 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "494")]
#[test_case(r#####"tw`to-neutral-900`"#####, r#####"({
  '--tw-gradient-to': "#171717 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "495")]
#[test_case(r#####"tw`to-stone-50`"#####, r#####"({
  '--tw-gradient-to': "#fafaf9 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "496")]
#[test_case(r#####"tw`to-stone-100`"#####, r#####"({
  '--tw-gradient-to': "#f5f5f4 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "497")]
#[test_case(r#####"tw`to-stone-200`"#####, r#####"({
  '--tw-gradient-to': "#e7e5e4 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "498")]
#[test_case(r#####"tw`to-stone-300`"#####, r#####"({
  '--tw-gradient-to': "#d6d3d1 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "499")]
#[test_case(r#####"tw`to-stone-400`"#####, r#####"({
  '--tw-gradient-to': "#a8a29e var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "500")]
#[test_case(r#####"tw`to-stone-500`"#####, r#####"({
  '--tw-gradient-to': "#78716c var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "501")]
#[test_case(r#####"tw`to-stone-600`"#####, r#####"({
  '--tw-gradient-to': "#57534e var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "502")]
#[test_case(r#####"tw`to-stone-700`"#####, r#####"({
  '--tw-gradient-to': "#44403c var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "503")]
#[test_case(r#####"tw`to-stone-800`"#####, r#####"({
  '--tw-gradient-to': "#292524 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "504")]
#[test_case(r#####"tw`to-stone-900`"#####, r#####"({
  '--tw-gradient-to': "#1c1917 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "505")]
#[test_case(r#####"tw`to-red-50`"#####, r#####"({
  '--tw-gradient-to': "#fef2f2 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "506")]
#[test_case(r#####"tw`to-red-100`"#####, r#####"({
  '--tw-gradient-to': "#fee2e2 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "507")]
#[test_case(r#####"tw`to-red-200`"#####, r#####"({
  '--tw-gradient-to': "#fecaca var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "508")]
#[test_case(r#####"tw`to-red-300`"#####, r#####"({
  '--tw-gradient-to': "#fca5a5 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "509")]
#[test_case(r#####"tw`to-red-400`"#####, r#####"({
  '--tw-gradient-to': "#f87171 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "510")]
#[test_case(r#####"tw`to-red-500`"#####, r#####"({
  '--tw-gradient-to': "#ef4444 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "511")]
#[test_case(r#####"tw`to-red-600`"#####, r#####"({
  '--tw-gradient-to': "#dc2626 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "512")]
#[test_case(r#####"tw`to-red-700`"#####, r#####"({
  '--tw-gradient-to': "#b91c1c var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "513")]
#[test_case(r#####"tw`to-red-800`"#####, r#####"({
  '--tw-gradient-to': "#991b1b var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "514")]
#[test_case(r#####"tw`to-red-900`"#####, r#####"({
  '--tw-gradient-to': "#7f1d1d var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "515")]
#[test_case(r#####"tw`to-orange-50`"#####, r#####"({
  '--tw-gradient-to': "#fff7ed var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "516")]
#[test_case(r#####"tw`to-orange-100`"#####, r#####"({
  '--tw-gradient-to': "#ffedd5 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "517")]
#[test_case(r#####"tw`to-orange-200`"#####, r#####"({
  '--tw-gradient-to': "#fed7aa var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "518")]
#[test_case(r#####"tw`to-orange-300`"#####, r#####"({
  '--tw-gradient-to': "#fdba74 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "519")]
#[test_case(r#####"tw`to-orange-400`"#####, r#####"({
  '--tw-gradient-to': "#fb923c var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "520")]
#[test_case(r#####"tw`to-orange-500`"#####, r#####"({
  '--tw-gradient-to': "#f97316 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "521")]
#[test_case(r#####"tw`to-orange-600`"#####, r#####"({
  '--tw-gradient-to': "#ea580c var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "522")]
#[test_case(r#####"tw`to-orange-700`"#####, r#####"({
  '--tw-gradient-to': "#c2410c var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "523")]
#[test_case(r#####"tw`to-orange-800`"#####, r#####"({
  '--tw-gradient-to': "#9a3412 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "524")]
#[test_case(r#####"tw`to-orange-900`"#####, r#####"({
  '--tw-gradient-to': "#7c2d12 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "525")]
#[test_case(r#####"tw`to-amber-50`"#####, r#####"({
  '--tw-gradient-to': "#fffbeb var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "526")]
#[test_case(r#####"tw`to-amber-100`"#####, r#####"({
  '--tw-gradient-to': "#fef3c7 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "527")]
#[test_case(r#####"tw`to-amber-200`"#####, r#####"({
  '--tw-gradient-to': "#fde68a var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "528")]
#[test_case(r#####"tw`to-amber-300`"#####, r#####"({
  '--tw-gradient-to': "#fcd34d var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "529")]
#[test_case(r#####"tw`to-amber-400`"#####, r#####"({
  '--tw-gradient-to': "#fbbf24 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "530")]
#[test_case(r#####"tw`to-amber-500`"#####, r#####"({
  '--tw-gradient-to': "#f59e0b var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "531")]
#[test_case(r#####"tw`to-amber-600`"#####, r#####"({
  '--tw-gradient-to': "#d97706 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "532")]
#[test_case(r#####"tw`to-amber-700`"#####, r#####"({
  '--tw-gradient-to': "#b45309 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "533")]
#[test_case(r#####"tw`to-amber-800`"#####, r#####"({
  '--tw-gradient-to': "#92400e var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "534")]
#[test_case(r#####"tw`to-amber-900`"#####, r#####"({
  '--tw-gradient-to': "#78350f var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "535")]
#[test_case(r#####"tw`to-yellow-50`"#####, r#####"({
  '--tw-gradient-to': "#fefce8 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "536")]
#[test_case(r#####"tw`to-yellow-100`"#####, r#####"({
  '--tw-gradient-to': "#fef9c3 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "537")]
#[test_case(r#####"tw`to-yellow-200`"#####, r#####"({
  '--tw-gradient-to': "#fef08a var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "538")]
#[test_case(r#####"tw`to-yellow-300`"#####, r#####"({
  '--tw-gradient-to': "#fde047 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "539")]
#[test_case(r#####"tw`to-yellow-400`"#####, r#####"({
  '--tw-gradient-to': "#facc15 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "540")]
#[test_case(r#####"tw`to-yellow-500`"#####, r#####"({
  '--tw-gradient-to': "#eab308 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "541")]
#[test_case(r#####"tw`to-yellow-600`"#####, r#####"({
  '--tw-gradient-to': "#ca8a04 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "542")]
#[test_case(r#####"tw`to-yellow-700`"#####, r#####"({
  '--tw-gradient-to': "#a16207 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "543")]
#[test_case(r#####"tw`to-yellow-800`"#####, r#####"({
  '--tw-gradient-to': "#854d0e var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "544")]
#[test_case(r#####"tw`to-yellow-900`"#####, r#####"({
  '--tw-gradient-to': "#713f12 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "545")]
#[test_case(r#####"tw`to-lime-50`"#####, r#####"({
  '--tw-gradient-to': "#f7fee7 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "546")]
#[test_case(r#####"tw`to-lime-100`"#####, r#####"({
  '--tw-gradient-to': "#ecfccb var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "547")]
#[test_case(r#####"tw`to-lime-200`"#####, r#####"({
  '--tw-gradient-to': "#d9f99d var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "548")]
#[test_case(r#####"tw`to-lime-300`"#####, r#####"({
  '--tw-gradient-to': "#bef264 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "549")]
#[test_case(r#####"tw`to-lime-400`"#####, r#####"({
  '--tw-gradient-to': "#a3e635 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "550")]
#[test_case(r#####"tw`to-lime-500`"#####, r#####"({
  '--tw-gradient-to': "#84cc16 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "551")]
#[test_case(r#####"tw`to-lime-600`"#####, r#####"({
  '--tw-gradient-to': "#65a30d var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "552")]
#[test_case(r#####"tw`to-lime-700`"#####, r#####"({
  '--tw-gradient-to': "#4d7c0f var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "553")]
#[test_case(r#####"tw`to-lime-800`"#####, r#####"({
  '--tw-gradient-to': "#3f6212 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "554")]
#[test_case(r#####"tw`to-lime-900`"#####, r#####"({
  '--tw-gradient-to': "#365314 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "555")]
#[test_case(r#####"tw`to-green-50`"#####, r#####"({
  '--tw-gradient-to': "#f0fdf4 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "556")]
#[test_case(r#####"tw`to-green-100`"#####, r#####"({
  '--tw-gradient-to': "#dcfce7 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "557")]
#[test_case(r#####"tw`to-green-200`"#####, r#####"({
  '--tw-gradient-to': "#bbf7d0 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "558")]
#[test_case(r#####"tw`to-green-300`"#####, r#####"({
  '--tw-gradient-to': "#86efac var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "559")]
#[test_case(r#####"tw`to-green-400`"#####, r#####"({
  '--tw-gradient-to': "#4ade80 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "560")]
#[test_case(r#####"tw`to-green-500`"#####, r#####"({
  '--tw-gradient-to': "#22c55e var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "561")]
#[test_case(r#####"tw`to-green-600`"#####, r#####"({
  '--tw-gradient-to': "#16a34a var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "562")]
#[test_case(r#####"tw`to-green-700`"#####, r#####"({
  '--tw-gradient-to': "#15803d var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "563")]
#[test_case(r#####"tw`to-green-800`"#####, r#####"({
  '--tw-gradient-to': "#166534 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "564")]
#[test_case(r#####"tw`to-green-900`"#####, r#####"({
  '--tw-gradient-to': "#14532d var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "565")]
#[test_case(r#####"tw`to-emerald-50`"#####, r#####"({
  '--tw-gradient-to': "#ecfdf5 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "566")]
#[test_case(r#####"tw`to-emerald-100`"#####, r#####"({
  '--tw-gradient-to': "#d1fae5 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "567")]
#[test_case(r#####"tw`to-emerald-200`"#####, r#####"({
  '--tw-gradient-to': "#a7f3d0 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "568")]
#[test_case(r#####"tw`to-emerald-300`"#####, r#####"({
  '--tw-gradient-to': "#6ee7b7 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "569")]
#[test_case(r#####"tw`to-emerald-400`"#####, r#####"({
  '--tw-gradient-to': "#34d399 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "570")]
#[test_case(r#####"tw`to-emerald-500`"#####, r#####"({
  '--tw-gradient-to': "#10b981 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "571")]
#[test_case(r#####"tw`to-emerald-600`"#####, r#####"({
  '--tw-gradient-to': "#059669 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "572")]
#[test_case(r#####"tw`to-emerald-700`"#####, r#####"({
  '--tw-gradient-to': "#047857 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "573")]
#[test_case(r#####"tw`to-emerald-800`"#####, r#####"({
  '--tw-gradient-to': "#065f46 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "574")]
#[test_case(r#####"tw`to-emerald-900`"#####, r#####"({
  '--tw-gradient-to': "#064e3b var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "575")]
#[test_case(r#####"tw`to-teal-50`"#####, r#####"({
  '--tw-gradient-to': "#f0fdfa var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "576")]
#[test_case(r#####"tw`to-teal-100`"#####, r#####"({
  '--tw-gradient-to': "#ccfbf1 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "577")]
#[test_case(r#####"tw`to-teal-200`"#####, r#####"({
  '--tw-gradient-to': "#99f6e4 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "578")]
#[test_case(r#####"tw`to-teal-300`"#####, r#####"({
  '--tw-gradient-to': "#5eead4 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "579")]
#[test_case(r#####"tw`to-teal-400`"#####, r#####"({
  '--tw-gradient-to': "#2dd4bf var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "580")]
#[test_case(r#####"tw`to-teal-500`"#####, r#####"({
  '--tw-gradient-to': "#14b8a6 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "581")]
#[test_case(r#####"tw`to-teal-600`"#####, r#####"({
  '--tw-gradient-to': "#0d9488 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "582")]
#[test_case(r#####"tw`to-teal-700`"#####, r#####"({
  '--tw-gradient-to': "#0f766e var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "583")]
#[test_case(r#####"tw`to-teal-800`"#####, r#####"({
  '--tw-gradient-to': "#115e59 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "584")]
#[test_case(r#####"tw`to-teal-900`"#####, r#####"({
  '--tw-gradient-to': "#134e4a var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "585")]
#[test_case(r#####"tw`to-cyan-50`"#####, r#####"({
  '--tw-gradient-to': "#ecfeff var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "586")]
#[test_case(r#####"tw`to-cyan-100`"#####, r#####"({
  '--tw-gradient-to': "#cffafe var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "587")]
#[test_case(r#####"tw`to-cyan-200`"#####, r#####"({
  '--tw-gradient-to': "#a5f3fc var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "588")]
#[test_case(r#####"tw`to-cyan-300`"#####, r#####"({
  '--tw-gradient-to': "#67e8f9 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "589")]
#[test_case(r#####"tw`to-cyan-400`"#####, r#####"({
  '--tw-gradient-to': "#22d3ee var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "590")]
#[test_case(r#####"tw`to-cyan-500`"#####, r#####"({
  '--tw-gradient-to': "#06b6d4 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "591")]
#[test_case(r#####"tw`to-cyan-600`"#####, r#####"({
  '--tw-gradient-to': "#0891b2 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "592")]
#[test_case(r#####"tw`to-cyan-700`"#####, r#####"({
  '--tw-gradient-to': "#0e7490 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "593")]
#[test_case(r#####"tw`to-cyan-800`"#####, r#####"({
  '--tw-gradient-to': "#155e75 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "594")]
#[test_case(r#####"tw`to-cyan-900`"#####, r#####"({
  '--tw-gradient-to': "#164e63 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "595")]
#[test_case(r#####"tw`to-sky-50`"#####, r#####"({
  '--tw-gradient-to': "#f0f9ff var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "596")]
#[test_case(r#####"tw`to-sky-100`"#####, r#####"({
  '--tw-gradient-to': "#e0f2fe var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "597")]
#[test_case(r#####"tw`to-sky-200`"#####, r#####"({
  '--tw-gradient-to': "#bae6fd var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "598")]
#[test_case(r#####"tw`to-sky-300`"#####, r#####"({
  '--tw-gradient-to': "#7dd3fc var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "599")]
#[test_case(r#####"tw`to-sky-400`"#####, r#####"({
  '--tw-gradient-to': "#38bdf8 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "600")]
#[test_case(r#####"tw`to-sky-500`"#####, r#####"({
  '--tw-gradient-to': "#0ea5e9 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "601")]
#[test_case(r#####"tw`to-sky-600`"#####, r#####"({
  '--tw-gradient-to': "#0284c7 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "602")]
#[test_case(r#####"tw`to-sky-700`"#####, r#####"({
  '--tw-gradient-to': "#0369a1 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "603")]
#[test_case(r#####"tw`to-sky-800`"#####, r#####"({
  '--tw-gradient-to': "#075985 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "604")]
#[test_case(r#####"tw`to-sky-900`"#####, r#####"({
  '--tw-gradient-to': "#0c4a6e var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "605")]
#[test_case(r#####"tw`to-blue-50`"#####, r#####"({
  '--tw-gradient-to': "#eff6ff var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "606")]
#[test_case(r#####"tw`to-blue-100`"#####, r#####"({
  '--tw-gradient-to': "#dbeafe var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "607")]
#[test_case(r#####"tw`to-blue-200`"#####, r#####"({
  '--tw-gradient-to': "#bfdbfe var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "608")]
#[test_case(r#####"tw`to-blue-300`"#####, r#####"({
  '--tw-gradient-to': "#93c5fd var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "609")]
#[test_case(r#####"tw`to-blue-400`"#####, r#####"({
  '--tw-gradient-to': "#60a5fa var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "610")]
#[test_case(r#####"tw`to-blue-500`"#####, r#####"({
  '--tw-gradient-to': "#3b82f6 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "611")]
#[test_case(r#####"tw`to-blue-600`"#####, r#####"({
  '--tw-gradient-to': "#2563eb var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "612")]
#[test_case(r#####"tw`to-blue-700`"#####, r#####"({
  '--tw-gradient-to': "#1d4ed8 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "613")]
#[test_case(r#####"tw`to-blue-800`"#####, r#####"({
  '--tw-gradient-to': "#1e40af var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "614")]
#[test_case(r#####"tw`to-blue-900`"#####, r#####"({
  '--tw-gradient-to': "#1e3a8a var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "615")]
#[test_case(r#####"tw`to-indigo-50`"#####, r#####"({
  '--tw-gradient-to': "#eef2ff var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "616")]
#[test_case(r#####"tw`to-indigo-100`"#####, r#####"({
  '--tw-gradient-to': "#e0e7ff var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "617")]
#[test_case(r#####"tw`to-indigo-200`"#####, r#####"({
  '--tw-gradient-to': "#c7d2fe var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "618")]
#[test_case(r#####"tw`to-indigo-300`"#####, r#####"({
  '--tw-gradient-to': "#a5b4fc var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "619")]
#[test_case(r#####"tw`to-indigo-400`"#####, r#####"({
  '--tw-gradient-to': "#818cf8 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "620")]
#[test_case(r#####"tw`to-indigo-500`"#####, r#####"({
  '--tw-gradient-to': "#6366f1 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "621")]
#[test_case(r#####"tw`to-indigo-600`"#####, r#####"({
  '--tw-gradient-to': "#4f46e5 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "622")]
#[test_case(r#####"tw`to-indigo-700`"#####, r#####"({
  '--tw-gradient-to': "#4338ca var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "623")]
#[test_case(r#####"tw`to-indigo-800`"#####, r#####"({
  '--tw-gradient-to': "#3730a3 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "624")]
#[test_case(r#####"tw`to-indigo-900`"#####, r#####"({
  '--tw-gradient-to': "#312e81 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "625")]
#[test_case(r#####"tw`to-violet-50`"#####, r#####"({
  '--tw-gradient-to': "#f5f3ff var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "626")]
#[test_case(r#####"tw`to-violet-100`"#####, r#####"({
  '--tw-gradient-to': "#ede9fe var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "627")]
#[test_case(r#####"tw`to-violet-200`"#####, r#####"({
  '--tw-gradient-to': "#ddd6fe var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "628")]
#[test_case(r#####"tw`to-violet-300`"#####, r#####"({
  '--tw-gradient-to': "#c4b5fd var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "629")]
#[test_case(r#####"tw`to-violet-400`"#####, r#####"({
  '--tw-gradient-to': "#a78bfa var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "630")]
#[test_case(r#####"tw`to-violet-500`"#####, r#####"({
  '--tw-gradient-to': "#8b5cf6 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "631")]
#[test_case(r#####"tw`to-violet-600`"#####, r#####"({
  '--tw-gradient-to': "#7c3aed var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "632")]
#[test_case(r#####"tw`to-violet-700`"#####, r#####"({
  '--tw-gradient-to': "#6d28d9 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "633")]
#[test_case(r#####"tw`to-violet-800`"#####, r#####"({
  '--tw-gradient-to': "#5b21b6 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "634")]
#[test_case(r#####"tw`to-violet-900`"#####, r#####"({
  '--tw-gradient-to': "#4c1d95 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "635")]
#[test_case(r#####"tw`to-purple-50`"#####, r#####"({
  '--tw-gradient-to': "#faf5ff var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "636")]
#[test_case(r#####"tw`to-purple-100`"#####, r#####"({
  '--tw-gradient-to': "#f3e8ff var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "637")]
#[test_case(r#####"tw`to-purple-200`"#####, r#####"({
  '--tw-gradient-to': "#e9d5ff var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "638")]
#[test_case(r#####"tw`to-purple-300`"#####, r#####"({
  '--tw-gradient-to': "#d8b4fe var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "639")]
#[test_case(r#####"tw`to-purple-400`"#####, r#####"({
  '--tw-gradient-to': "#c084fc var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "640")]
#[test_case(r#####"tw`to-purple-500`"#####, r#####"({
  '--tw-gradient-to': "#a855f7 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "641")]
#[test_case(r#####"tw`to-purple-600`"#####, r#####"({
  '--tw-gradient-to': "#9333ea var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "642")]
#[test_case(r#####"tw`to-purple-700`"#####, r#####"({
  '--tw-gradient-to': "#7e22ce var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "643")]
#[test_case(r#####"tw`to-purple-800`"#####, r#####"({
  '--tw-gradient-to': "#6b21a8 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "644")]
#[test_case(r#####"tw`to-purple-900`"#####, r#####"({
  '--tw-gradient-to': "#581c87 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "645")]
#[test_case(r#####"tw`to-fuchsia-50`"#####, r#####"({
  '--tw-gradient-to': "#fdf4ff var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "646")]
#[test_case(r#####"tw`to-fuchsia-100`"#####, r#####"({
  '--tw-gradient-to': "#fae8ff var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "647")]
#[test_case(r#####"tw`to-fuchsia-200`"#####, r#####"({
  '--tw-gradient-to': "#f5d0fe var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "648")]
#[test_case(r#####"tw`to-fuchsia-300`"#####, r#####"({
  '--tw-gradient-to': "#f0abfc var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "649")]
#[test_case(r#####"tw`to-fuchsia-400`"#####, r#####"({
  '--tw-gradient-to': "#e879f9 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "650")]
#[test_case(r#####"tw`to-fuchsia-500`"#####, r#####"({
  '--tw-gradient-to': "#d946ef var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "651")]
#[test_case(r#####"tw`to-fuchsia-600`"#####, r#####"({
  '--tw-gradient-to': "#c026d3 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "652")]
#[test_case(r#####"tw`to-fuchsia-700`"#####, r#####"({
  '--tw-gradient-to': "#a21caf var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "653")]
#[test_case(r#####"tw`to-fuchsia-800`"#####, r#####"({
  '--tw-gradient-to': "#86198f var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "654")]
#[test_case(r#####"tw`to-fuchsia-900`"#####, r#####"({
  '--tw-gradient-to': "#701a75 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "655")]
#[test_case(r#####"tw`to-pink-50`"#####, r#####"({
  '--tw-gradient-to': "#fdf2f8 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "656")]
#[test_case(r#####"tw`to-pink-100`"#####, r#####"({
  '--tw-gradient-to': "#fce7f3 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "657")]
#[test_case(r#####"tw`to-pink-200`"#####, r#####"({
  '--tw-gradient-to': "#fbcfe8 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "658")]
#[test_case(r#####"tw`to-pink-300`"#####, r#####"({
  '--tw-gradient-to': "#f9a8d4 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "659")]
#[test_case(r#####"tw`to-pink-400`"#####, r#####"({
  '--tw-gradient-to': "#f472b6 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "660")]
#[test_case(r#####"tw`to-pink-500`"#####, r#####"({
  '--tw-gradient-to': "#ec4899 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "661")]
#[test_case(r#####"tw`to-pink-600`"#####, r#####"({
  '--tw-gradient-to': "#db2777 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "662")]
#[test_case(r#####"tw`to-pink-700`"#####, r#####"({
  '--tw-gradient-to': "#be185d var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "663")]
#[test_case(r#####"tw`to-pink-800`"#####, r#####"({
  '--tw-gradient-to': "#9d174d var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "664")]
#[test_case(r#####"tw`to-pink-900`"#####, r#####"({
  '--tw-gradient-to': "#831843 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "665")]
#[test_case(r#####"tw`to-rose-50`"#####, r#####"({
  '--tw-gradient-to': "#fff1f2 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "666")]
#[test_case(r#####"tw`to-rose-100`"#####, r#####"({
  '--tw-gradient-to': "#ffe4e6 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "667")]
#[test_case(r#####"tw`to-rose-200`"#####, r#####"({
  '--tw-gradient-to': "#fecdd3 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "668")]
#[test_case(r#####"tw`to-rose-300`"#####, r#####"({
  '--tw-gradient-to': "#fda4af var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "669")]
#[test_case(r#####"tw`to-rose-400`"#####, r#####"({
  '--tw-gradient-to': "#fb7185 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "670")]
#[test_case(r#####"tw`to-rose-500`"#####, r#####"({
  '--tw-gradient-to': "#f43f5e var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "671")]
#[test_case(r#####"tw`to-rose-600`"#####, r#####"({
  '--tw-gradient-to': "#e11d48 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "672")]
#[test_case(r#####"tw`to-rose-700`"#####, r#####"({
  '--tw-gradient-to': "#be123c var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "673")]
#[test_case(r#####"tw`to-rose-800`"#####, r#####"({
  '--tw-gradient-to': "#9f1239 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "674")]
#[test_case(r#####"tw`to-rose-900`"#####, r#####"({
  '--tw-gradient-to': "#881337 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "675")]
#[test_case(r#####"tw`bg-gradient-to-t from-electric to-electric text-purple-500 text-opacity-50`"#####, r#####"({
  backgroundImage: "linear-gradient(to top, var(--tw-gradient-stops))",
  '--tw-gradient-from': "rgb(219, 0, 255) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(219, 0, 255) var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
  '--tw-text-opacity': "0.5",
  color: "rgb(168 85 247 / var(--tw-text-opacity))",
})
;"##### ; "676")]
#[test_case(r#####"tw`bg-gradient-to-r from-indigo-500`"#####, r#####"({
  backgroundImage: "linear-gradient(to right, var(--tw-gradient-stops))",
  '--tw-gradient-from': "#6366f1 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(99 102 241 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "677")]
#[test_case(r#####"tw`bg-gradient-to-r from-cyan-500 to-blue-500`"#####, r#####"({
  backgroundImage: "linear-gradient(to right, var(--tw-gradient-stops))",
  '--tw-gradient-from': "#06b6d4 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "#3b82f6 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "678")]
#[test_case(r#####"tw`bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500`"#####, r#####"({
  backgroundImage: "linear-gradient(to right, var(--tw-gradient-stops))",
  '--tw-gradient-from': "#6366f1 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "#ec4899 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #a855f7 var(--tw-gradient-via-position), var(--tw-gradient-to)",
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "679")]
#[test_case(r#####"tw`bg-gradient-to-r from-green-400 to-blue-500 hover:from-pink-500 hover:to-yellow-500`"#####, r#####"({
  backgroundImage: "linear-gradient(to right, var(--tw-gradient-stops))",
  '--tw-gradient-from': "#4ade80 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "#3b82f6 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
  ':hover': {
    '--tw-gradient-from': "#ec4899 var(--tw-gradient-from-position)",
    '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
    '--tw-gradient-to': "#eab308 var(--tw-gradient-to-position)",
    '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
    '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
  },
})
;"##### ; "680")]
#[test_case(r#####"tw`bg-gradient-to-r from-purple-400 md:from-yellow-500`"#####, r#####"({
  backgroundImage: "linear-gradient(to right, var(--tw-gradient-stops))",
  '--tw-gradient-from': "#c084fc var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(192 132 252 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
  '@media (min-width: 768px)': {
    '--tw-gradient-from': "#eab308 var(--tw-gradient-from-position)",
    '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
    '--tw-gradient-to': "rgb(234 179 8 / 0)  var(--tw-gradient-from-position)",
    '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
    '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
  },
})
;"##### ; "681")]
#[test_case(r#####"tw`from-[#da5b66] via-[#da5b66] to-[#da5b66]`"#####, r#####"({
  '--tw-gradient-from': "#da5b66 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "#da5b66 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #da5b66 var(--tw-gradient-via-position), var(--tw-gradient-to)",
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "682")]
#[test_case(r#####"tw`from-[var(--color)] via-[var(--color)] to-[var(--color)]`"#####, r#####"({
  '--tw-gradient-from': "var(--color) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "var(--color) var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), var(--color) var(--tw-gradient-via-position), var(--tw-gradient-to)",
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "683")]
#[test_case(r#####"tw`from-red-500`"#####, r#####"({
  '--tw-gradient-from': "#ef4444 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 68 68 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "684")]
#[test_case(r#####"tw`from-red-500/25`"#####, r#####"({
  '--tw-gradient-from':
    "rgb(239 68 68 / 0.25) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 68 68 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "685")]
#[test_case(r#####"tw`from-red-500/fromConfig`"#####, r#####"({
  '--tw-gradient-from': "#000 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(0 0 0 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "686")]
#[test_case(r#####"tw`from-red-500/fromConfig/25`"#####, r#####"({
  '--tw-gradient-from': "rgb(0 0 0 / 0.25) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(0 0 0 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "687")]
#[test_case(r#####"tw`from-red-500/fromConfig/[.555]`"#####, r#####"({
  '--tw-gradient-from': "rgb(0 0 0 / .555) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(0 0 0 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "688")]
#[test_case(r#####"tw`from-red-500/fromConfig/[var(--myvar)]`"#####, r#####"({
  '--tw-gradient-from':
    "rgb(0 0 0 / var(--myvar)) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(0 0 0 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "689")]
#[test_case(r#####"tw`from-red-500/[.555]`"#####, r#####"({
  '--tw-gradient-from':
    "rgb(239 68 68 / .555) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 68 68 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "690")]
#[test_case(r#####"tw`from-red-500/[var(--myvar)]`"#####, r#####"({
  '--tw-gradient-from':
    "rgb(239 68 68 / var(--myvar)) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 68 68 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "691")]
#[test_case(r#####"tw`from-[theme('colors.red.500')]`"#####, r#####"({
  '--tw-gradient-from': "#ef4444 var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 68 68 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "692")]
#[test_case(r#####"tw`from-electric`"#####, r#####"({
  '--tw-gradient-from': "rgb(219, 0, 255) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgba(219, 0, 255, 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "693")]
#[test_case(r#####"tw`from-electric/25`"#####, r#####"({
  '--tw-gradient-from':
    "rgba(219, 0, 255, 0.25) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgba(219, 0, 255, 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "694")]
#[test_case(r#####"tw`from-electric/[.555]`"#####, r#####"({
  '--tw-gradient-from':
    "rgba(219, 0, 255, .555) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgba(219, 0, 255, 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "695")]
#[test_case(r#####"tw`from-electric/[var(--myvar)]`"#####, r#####"({
  '--tw-gradient-from':
    "rgba(219, 0, 255, var(--myvar)) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgba(219, 0, 255, 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "696")]
#[test_case(r#####"tw`from-[theme('colors.electric')]`"#####, r#####"({
  '--tw-gradient-from': "rgb(219, 0, 255) var(--tw-gradient-from-position)",
  '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(219 0 255 / 0)  var(--tw-gradient-from-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops': "var(--tw-gradient-from), var(--tw-gradient-to)",
})
;"##### ; "697")]
#[test_case(r#####"tw`via-red-500`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 68 68 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ef4444 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "698")]
#[test_case(r#####"tw`via-red-500/25`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 68 68 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), rgb(239 68 68 / 0.25) var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "699")]
#[test_case(r#####"tw`via-red-500/fromConfig`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(0 0 0 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #000 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "700")]
#[test_case(r#####"tw`via-red-500/[.555]`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 68 68 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), rgb(239 68 68 / .555) var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "701")]
#[test_case(r#####"tw`via-red-500/[var(--myvar)]`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 68 68 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), rgb(239 68 68 / var(--myvar)) var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "702")]
#[test_case(r#####"tw`via-[theme('colors.red.500')]`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(239 68 68 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), #ef4444 var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "703")]
#[test_case(r#####"tw`via-electric`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgba(219, 0, 255, 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), rgb(219, 0, 255) var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "704")]
#[test_case(r#####"tw`via-electric/25`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgba(219, 0, 255, 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), rgba(219, 0, 255, 0.25) var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "705")]
#[test_case(r#####"tw`via-electric/[.555]`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgba(219, 0, 255, 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), rgba(219, 0, 255, .555) var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "706")]
#[test_case(r#####"tw`via-electric/[var(--myvar)]`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgba(219, 0, 255, 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), rgba(219, 0, 255, var(--myvar)) var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "707")]
#[test_case(r#####"tw`via-[theme('colors.electric')]`"#####, r#####"({
  '--tw-gradient-via-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-to': "rgb(219 0 255 / 0)  var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '--tw-gradient-stops':
    "var(--tw-gradient-from), rgb(219, 0, 255) var(--tw-gradient-via-position), var(--tw-gradient-to)",
})
;"##### ; "708")]
#[test_case(r#####"tw`to-red-500/25`"#####, r#####"({
  '--tw-gradient-to': "rgb(239 68 68 / 0.25) var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "709")]
#[test_case(r#####"tw`to-red-500/fromConfig`"#####, r#####"({
  '--tw-gradient-to': "#000 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "710")]
#[test_case(r#####"tw`to-red-500/[.555]`"#####, r#####"({
  '--tw-gradient-to': "rgb(239 68 68 / .555) var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "711")]
#[test_case(r#####"tw`to-red-500/[var(--myvar)]`"#####, r#####"({
  '--tw-gradient-to':
    "rgb(239 68 68 / var(--myvar)) var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "712")]
#[test_case(r#####"tw`to-[theme('colors.red.500')]`"#####, r#####"({
  '--tw-gradient-to': "#ef4444 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "713")]
#[test_case(r#####"tw`to-electric`"#####, r#####"({
  '--tw-gradient-to': "rgb(219, 0, 255) var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "714")]
#[test_case(r#####"tw`to-electric/25`"#####, r#####"({
  '--tw-gradient-to': "rgba(219, 0, 255, 0.25) var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "715")]
#[test_case(r#####"tw`to-electric/[.555]`"#####, r#####"({
  '--tw-gradient-to': "rgba(219, 0, 255, .555) var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "716")]
#[test_case(r#####"tw`to-electric/[var(--myvar)]`"#####, r#####"({
  '--tw-gradient-to':
    "rgba(219, 0, 255, var(--myvar)) var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})
;"##### ; "717")]
#[test_case(r#####"tw`to-[theme('colors.electric')]`"#####, r#####"({
  '--tw-gradient-to': "rgb(219, 0, 255) var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
})"##### ; "718")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
