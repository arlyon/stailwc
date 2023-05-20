use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`outlineColor`"#####, r#####"({
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
;"##### ; "1")]
#[test_case(r#####"tw`outline-inherit`"#####, r#####"({
  outlineColor: "inherit",
})
;"##### ; "2")]
#[test_case(r#####"tw`outline-current`"#####, r#####"({
  outlineColor: "currentColor",
})
;"##### ; "3")]
#[test_case(r#####"tw`outline-transparent`"#####, r#####"({
  outlineColor: "transparent",
})
;"##### ; "4")]
#[test_case(r#####"tw`outline-black`"#####, r#####"({
  outlineColor: "#000",
})
;"##### ; "5")]
#[test_case(r#####"tw`outline-white`"#####, r#####"({
  outlineColor: "#fff",
})
;"##### ; "6")]
#[test_case(r#####"tw`outline-slate-50`"#####, r#####"({
  outlineColor: "#f8fafc",
})
;"##### ; "7")]
#[test_case(r#####"tw`outline-slate-100`"#####, r#####"({
  outlineColor: "#f1f5f9",
})
;"##### ; "8")]
#[test_case(r#####"tw`outline-slate-200`"#####, r#####"({
  outlineColor: "#e2e8f0",
})
;"##### ; "9")]
#[test_case(r#####"tw`outline-slate-300`"#####, r#####"({
  outlineColor: "#cbd5e1",
})
;"##### ; "10")]
#[test_case(r#####"tw`outline-slate-400`"#####, r#####"({
  outlineColor: "#94a3b8",
})
;"##### ; "11")]
#[test_case(r#####"tw`outline-slate-500`"#####, r#####"({
  outlineColor: "#64748b",
})
;"##### ; "12")]
#[test_case(r#####"tw`outline-slate-600`"#####, r#####"({
  outlineColor: "#475569",
})
;"##### ; "13")]
#[test_case(r#####"tw`outline-slate-700`"#####, r#####"({
  outlineColor: "#334155",
})
;"##### ; "14")]
#[test_case(r#####"tw`outline-slate-800`"#####, r#####"({
  outlineColor: "#1e293b",
})
;"##### ; "15")]
#[test_case(r#####"tw`outline-slate-900`"#####, r#####"({
  outlineColor: "#0f172a",
})
;"##### ; "16")]
#[test_case(r#####"tw`outline-gray-50`"#####, r#####"({
  outlineColor: "#f9fafb",
})
;"##### ; "17")]
#[test_case(r#####"tw`outline-gray-100`"#####, r#####"({
  outlineColor: "#f3f4f6",
})
;"##### ; "18")]
#[test_case(r#####"tw`outline-gray-200`"#####, r#####"({
  outlineColor: "#e5e7eb",
})
;"##### ; "19")]
#[test_case(r#####"tw`outline-gray-300`"#####, r#####"({
  outlineColor: "#d1d5db",
})
;"##### ; "20")]
#[test_case(r#####"tw`outline-gray-400`"#####, r#####"({
  outlineColor: "#9ca3af",
})
;"##### ; "21")]
#[test_case(r#####"tw`outline-gray-500`"#####, r#####"({
  outlineColor: "#6b7280",
})
;"##### ; "22")]
#[test_case(r#####"tw`outline-gray-600`"#####, r#####"({
  outlineColor: "#4b5563",
})
;"##### ; "23")]
#[test_case(r#####"tw`outline-gray-700`"#####, r#####"({
  outlineColor: "#374151",
})
;"##### ; "24")]
#[test_case(r#####"tw`outline-gray-800`"#####, r#####"({
  outlineColor: "#1f2937",
})
;"##### ; "25")]
#[test_case(r#####"tw`outline-gray-900`"#####, r#####"({
  outlineColor: "#111827",
})
;"##### ; "26")]
#[test_case(r#####"tw`outline-zinc-50`"#####, r#####"({
  outlineColor: "#fafafa",
})
;"##### ; "27")]
#[test_case(r#####"tw`outline-zinc-100`"#####, r#####"({
  outlineColor: "#f4f4f5",
})
;"##### ; "28")]
#[test_case(r#####"tw`outline-zinc-200`"#####, r#####"({
  outlineColor: "#e4e4e7",
})
;"##### ; "29")]
#[test_case(r#####"tw`outline-zinc-300`"#####, r#####"({
  outlineColor: "#d4d4d8",
})
;"##### ; "30")]
#[test_case(r#####"tw`outline-zinc-400`"#####, r#####"({
  outlineColor: "#a1a1aa",
})
;"##### ; "31")]
#[test_case(r#####"tw`outline-zinc-500`"#####, r#####"({
  outlineColor: "#71717a",
})
;"##### ; "32")]
#[test_case(r#####"tw`outline-zinc-600`"#####, r#####"({
  outlineColor: "#52525b",
})
;"##### ; "33")]
#[test_case(r#####"tw`outline-zinc-700`"#####, r#####"({
  outlineColor: "#3f3f46",
})
;"##### ; "34")]
#[test_case(r#####"tw`outline-zinc-800`"#####, r#####"({
  outlineColor: "#27272a",
})
;"##### ; "35")]
#[test_case(r#####"tw`outline-zinc-900`"#####, r#####"({
  outlineColor: "#18181b",
})
;"##### ; "36")]
#[test_case(r#####"tw`outline-neutral-50`"#####, r#####"({
  outlineColor: "#fafafa",
})
;"##### ; "37")]
#[test_case(r#####"tw`outline-neutral-100`"#####, r#####"({
  outlineColor: "#f5f5f5",
})
;"##### ; "38")]
#[test_case(r#####"tw`outline-neutral-200`"#####, r#####"({
  outlineColor: "#e5e5e5",
})
;"##### ; "39")]
#[test_case(r#####"tw`outline-neutral-300`"#####, r#####"({
  outlineColor: "#d4d4d4",
})
;"##### ; "40")]
#[test_case(r#####"tw`outline-neutral-400`"#####, r#####"({
  outlineColor: "#a3a3a3",
})
;"##### ; "41")]
#[test_case(r#####"tw`outline-neutral-500`"#####, r#####"({
  outlineColor: "#737373",
})
;"##### ; "42")]
#[test_case(r#####"tw`outline-neutral-600`"#####, r#####"({
  outlineColor: "#525252",
})
;"##### ; "43")]
#[test_case(r#####"tw`outline-neutral-700`"#####, r#####"({
  outlineColor: "#404040",
})
;"##### ; "44")]
#[test_case(r#####"tw`outline-neutral-800`"#####, r#####"({
  outlineColor: "#262626",
})
;"##### ; "45")]
#[test_case(r#####"tw`outline-neutral-900`"#####, r#####"({
  outlineColor: "#171717",
})
;"##### ; "46")]
#[test_case(r#####"tw`outline-stone-50`"#####, r#####"({
  outlineColor: "#fafaf9",
})
;"##### ; "47")]
#[test_case(r#####"tw`outline-stone-100`"#####, r#####"({
  outlineColor: "#f5f5f4",
})
;"##### ; "48")]
#[test_case(r#####"tw`outline-stone-200`"#####, r#####"({
  outlineColor: "#e7e5e4",
})
;"##### ; "49")]
#[test_case(r#####"tw`outline-stone-300`"#####, r#####"({
  outlineColor: "#d6d3d1",
})
;"##### ; "50")]
#[test_case(r#####"tw`outline-stone-400`"#####, r#####"({
  outlineColor: "#a8a29e",
})
;"##### ; "51")]
#[test_case(r#####"tw`outline-stone-500`"#####, r#####"({
  outlineColor: "#78716c",
})
;"##### ; "52")]
#[test_case(r#####"tw`outline-stone-600`"#####, r#####"({
  outlineColor: "#57534e",
})
;"##### ; "53")]
#[test_case(r#####"tw`outline-stone-700`"#####, r#####"({
  outlineColor: "#44403c",
})
;"##### ; "54")]
#[test_case(r#####"tw`outline-stone-800`"#####, r#####"({
  outlineColor: "#292524",
})
;"##### ; "55")]
#[test_case(r#####"tw`outline-stone-900`"#####, r#####"({
  outlineColor: "#1c1917",
})
;"##### ; "56")]
#[test_case(r#####"tw`outline-red-50`"#####, r#####"({
  outlineColor: "#fef2f2",
})
;"##### ; "57")]
#[test_case(r#####"tw`outline-red-100`"#####, r#####"({
  outlineColor: "#fee2e2",
})
;"##### ; "58")]
#[test_case(r#####"tw`outline-red-200`"#####, r#####"({
  outlineColor: "#fecaca",
})
;"##### ; "59")]
#[test_case(r#####"tw`outline-red-300`"#####, r#####"({
  outlineColor: "#fca5a5",
})
;"##### ; "60")]
#[test_case(r#####"tw`outline-red-400`"#####, r#####"({
  outlineColor: "#f87171",
})
;"##### ; "61")]
#[test_case(r#####"tw`outline-red-500`"#####, r#####"({
  outlineColor: "#ef4444",
})
;"##### ; "62")]
#[test_case(r#####"tw`outline-red-600`"#####, r#####"({
  outlineColor: "#dc2626",
})
;"##### ; "63")]
#[test_case(r#####"tw`outline-red-700`"#####, r#####"({
  outlineColor: "#b91c1c",
})
;"##### ; "64")]
#[test_case(r#####"tw`outline-red-800`"#####, r#####"({
  outlineColor: "#991b1b",
})
;"##### ; "65")]
#[test_case(r#####"tw`outline-red-900`"#####, r#####"({
  outlineColor: "#7f1d1d",
})
;"##### ; "66")]
#[test_case(r#####"tw`outline-orange-50`"#####, r#####"({
  outlineColor: "#fff7ed",
})
;"##### ; "67")]
#[test_case(r#####"tw`outline-orange-100`"#####, r#####"({
  outlineColor: "#ffedd5",
})
;"##### ; "68")]
#[test_case(r#####"tw`outline-orange-200`"#####, r#####"({
  outlineColor: "#fed7aa",
})
;"##### ; "69")]
#[test_case(r#####"tw`outline-orange-300`"#####, r#####"({
  outlineColor: "#fdba74",
})
;"##### ; "70")]
#[test_case(r#####"tw`outline-orange-400`"#####, r#####"({
  outlineColor: "#fb923c",
})
;"##### ; "71")]
#[test_case(r#####"tw`outline-orange-500`"#####, r#####"({
  outlineColor: "#f97316",
})
;"##### ; "72")]
#[test_case(r#####"tw`outline-orange-600`"#####, r#####"({
  outlineColor: "#ea580c",
})
;"##### ; "73")]
#[test_case(r#####"tw`outline-orange-700`"#####, r#####"({
  outlineColor: "#c2410c",
})
;"##### ; "74")]
#[test_case(r#####"tw`outline-orange-800`"#####, r#####"({
  outlineColor: "#9a3412",
})
;"##### ; "75")]
#[test_case(r#####"tw`outline-orange-900`"#####, r#####"({
  outlineColor: "#7c2d12",
})
;"##### ; "76")]
#[test_case(r#####"tw`outline-amber-50`"#####, r#####"({
  outlineColor: "#fffbeb",
})
;"##### ; "77")]
#[test_case(r#####"tw`outline-amber-100`"#####, r#####"({
  outlineColor: "#fef3c7",
})
;"##### ; "78")]
#[test_case(r#####"tw`outline-amber-200`"#####, r#####"({
  outlineColor: "#fde68a",
})
;"##### ; "79")]
#[test_case(r#####"tw`outline-amber-300`"#####, r#####"({
  outlineColor: "#fcd34d",
})
;"##### ; "80")]
#[test_case(r#####"tw`outline-amber-400`"#####, r#####"({
  outlineColor: "#fbbf24",
})
;"##### ; "81")]
#[test_case(r#####"tw`outline-amber-500`"#####, r#####"({
  outlineColor: "#f59e0b",
})
;"##### ; "82")]
#[test_case(r#####"tw`outline-amber-600`"#####, r#####"({
  outlineColor: "#d97706",
})
;"##### ; "83")]
#[test_case(r#####"tw`outline-amber-700`"#####, r#####"({
  outlineColor: "#b45309",
})
;"##### ; "84")]
#[test_case(r#####"tw`outline-amber-800`"#####, r#####"({
  outlineColor: "#92400e",
})
;"##### ; "85")]
#[test_case(r#####"tw`outline-amber-900`"#####, r#####"({
  outlineColor: "#78350f",
})
;"##### ; "86")]
#[test_case(r#####"tw`outline-yellow-50`"#####, r#####"({
  outlineColor: "#fefce8",
})
;"##### ; "87")]
#[test_case(r#####"tw`outline-yellow-100`"#####, r#####"({
  outlineColor: "#fef9c3",
})
;"##### ; "88")]
#[test_case(r#####"tw`outline-yellow-200`"#####, r#####"({
  outlineColor: "#fef08a",
})
;"##### ; "89")]
#[test_case(r#####"tw`outline-yellow-300`"#####, r#####"({
  outlineColor: "#fde047",
})
;"##### ; "90")]
#[test_case(r#####"tw`outline-yellow-400`"#####, r#####"({
  outlineColor: "#facc15",
})
;"##### ; "91")]
#[test_case(r#####"tw`outline-yellow-500`"#####, r#####"({
  outlineColor: "#eab308",
})
;"##### ; "92")]
#[test_case(r#####"tw`outline-yellow-600`"#####, r#####"({
  outlineColor: "#ca8a04",
})
;"##### ; "93")]
#[test_case(r#####"tw`outline-yellow-700`"#####, r#####"({
  outlineColor: "#a16207",
})
;"##### ; "94")]
#[test_case(r#####"tw`outline-yellow-800`"#####, r#####"({
  outlineColor: "#854d0e",
})
;"##### ; "95")]
#[test_case(r#####"tw`outline-yellow-900`"#####, r#####"({
  outlineColor: "#713f12",
})
;"##### ; "96")]
#[test_case(r#####"tw`outline-lime-50`"#####, r#####"({
  outlineColor: "#f7fee7",
})
;"##### ; "97")]
#[test_case(r#####"tw`outline-lime-100`"#####, r#####"({
  outlineColor: "#ecfccb",
})
;"##### ; "98")]
#[test_case(r#####"tw`outline-lime-200`"#####, r#####"({
  outlineColor: "#d9f99d",
})
;"##### ; "99")]
#[test_case(r#####"tw`outline-lime-300`"#####, r#####"({
  outlineColor: "#bef264",
})
;"##### ; "100")]
#[test_case(r#####"tw`outline-lime-400`"#####, r#####"({
  outlineColor: "#a3e635",
})
;"##### ; "101")]
#[test_case(r#####"tw`outline-lime-500`"#####, r#####"({
  outlineColor: "#84cc16",
})
;"##### ; "102")]
#[test_case(r#####"tw`outline-lime-600`"#####, r#####"({
  outlineColor: "#65a30d",
})
;"##### ; "103")]
#[test_case(r#####"tw`outline-lime-700`"#####, r#####"({
  outlineColor: "#4d7c0f",
})
;"##### ; "104")]
#[test_case(r#####"tw`outline-lime-800`"#####, r#####"({
  outlineColor: "#3f6212",
})
;"##### ; "105")]
#[test_case(r#####"tw`outline-lime-900`"#####, r#####"({
  outlineColor: "#365314",
})
;"##### ; "106")]
#[test_case(r#####"tw`outline-green-50`"#####, r#####"({
  outlineColor: "#f0fdf4",
})
;"##### ; "107")]
#[test_case(r#####"tw`outline-green-100`"#####, r#####"({
  outlineColor: "#dcfce7",
})
;"##### ; "108")]
#[test_case(r#####"tw`outline-green-200`"#####, r#####"({
  outlineColor: "#bbf7d0",
})
;"##### ; "109")]
#[test_case(r#####"tw`outline-green-300`"#####, r#####"({
  outlineColor: "#86efac",
})
;"##### ; "110")]
#[test_case(r#####"tw`outline-green-400`"#####, r#####"({
  outlineColor: "#4ade80",
})
;"##### ; "111")]
#[test_case(r#####"tw`outline-green-500`"#####, r#####"({
  outlineColor: "#22c55e",
})
;"##### ; "112")]
#[test_case(r#####"tw`outline-green-600`"#####, r#####"({
  outlineColor: "#16a34a",
})
;"##### ; "113")]
#[test_case(r#####"tw`outline-green-700`"#####, r#####"({
  outlineColor: "#15803d",
})
;"##### ; "114")]
#[test_case(r#####"tw`outline-green-800`"#####, r#####"({
  outlineColor: "#166534",
})
;"##### ; "115")]
#[test_case(r#####"tw`outline-green-900`"#####, r#####"({
  outlineColor: "#14532d",
})
;"##### ; "116")]
#[test_case(r#####"tw`outline-emerald-50`"#####, r#####"({
  outlineColor: "#ecfdf5",
})
;"##### ; "117")]
#[test_case(r#####"tw`outline-emerald-100`"#####, r#####"({
  outlineColor: "#d1fae5",
})
;"##### ; "118")]
#[test_case(r#####"tw`outline-emerald-200`"#####, r#####"({
  outlineColor: "#a7f3d0",
})
;"##### ; "119")]
#[test_case(r#####"tw`outline-emerald-300`"#####, r#####"({
  outlineColor: "#6ee7b7",
})
;"##### ; "120")]
#[test_case(r#####"tw`outline-emerald-400`"#####, r#####"({
  outlineColor: "#34d399",
})
;"##### ; "121")]
#[test_case(r#####"tw`outline-emerald-500`"#####, r#####"({
  outlineColor: "#10b981",
})
;"##### ; "122")]
#[test_case(r#####"tw`outline-emerald-600`"#####, r#####"({
  outlineColor: "#059669",
})
;"##### ; "123")]
#[test_case(r#####"tw`outline-emerald-700`"#####, r#####"({
  outlineColor: "#047857",
})
;"##### ; "124")]
#[test_case(r#####"tw`outline-emerald-800`"#####, r#####"({
  outlineColor: "#065f46",
})
;"##### ; "125")]
#[test_case(r#####"tw`outline-emerald-900`"#####, r#####"({
  outlineColor: "#064e3b",
})
;"##### ; "126")]
#[test_case(r#####"tw`outline-teal-50`"#####, r#####"({
  outlineColor: "#f0fdfa",
})
;"##### ; "127")]
#[test_case(r#####"tw`outline-teal-100`"#####, r#####"({
  outlineColor: "#ccfbf1",
})
;"##### ; "128")]
#[test_case(r#####"tw`outline-teal-200`"#####, r#####"({
  outlineColor: "#99f6e4",
})
;"##### ; "129")]
#[test_case(r#####"tw`outline-teal-300`"#####, r#####"({
  outlineColor: "#5eead4",
})
;"##### ; "130")]
#[test_case(r#####"tw`outline-teal-400`"#####, r#####"({
  outlineColor: "#2dd4bf",
})
;"##### ; "131")]
#[test_case(r#####"tw`outline-teal-500`"#####, r#####"({
  outlineColor: "#14b8a6",
})
;"##### ; "132")]
#[test_case(r#####"tw`outline-teal-600`"#####, r#####"({
  outlineColor: "#0d9488",
})
;"##### ; "133")]
#[test_case(r#####"tw`outline-teal-700`"#####, r#####"({
  outlineColor: "#0f766e",
})
;"##### ; "134")]
#[test_case(r#####"tw`outline-teal-800`"#####, r#####"({
  outlineColor: "#115e59",
})
;"##### ; "135")]
#[test_case(r#####"tw`outline-teal-900`"#####, r#####"({
  outlineColor: "#134e4a",
})
;"##### ; "136")]
#[test_case(r#####"tw`outline-cyan-50`"#####, r#####"({
  outlineColor: "#ecfeff",
})
;"##### ; "137")]
#[test_case(r#####"tw`outline-cyan-100`"#####, r#####"({
  outlineColor: "#cffafe",
})
;"##### ; "138")]
#[test_case(r#####"tw`outline-cyan-200`"#####, r#####"({
  outlineColor: "#a5f3fc",
})
;"##### ; "139")]
#[test_case(r#####"tw`outline-cyan-300`"#####, r#####"({
  outlineColor: "#67e8f9",
})
;"##### ; "140")]
#[test_case(r#####"tw`outline-cyan-400`"#####, r#####"({
  outlineColor: "#22d3ee",
})
;"##### ; "141")]
#[test_case(r#####"tw`outline-cyan-500`"#####, r#####"({
  outlineColor: "#06b6d4",
})
;"##### ; "142")]
#[test_case(r#####"tw`outline-cyan-600`"#####, r#####"({
  outlineColor: "#0891b2",
})
;"##### ; "143")]
#[test_case(r#####"tw`outline-cyan-700`"#####, r#####"({
  outlineColor: "#0e7490",
})
;"##### ; "144")]
#[test_case(r#####"tw`outline-cyan-800`"#####, r#####"({
  outlineColor: "#155e75",
})
;"##### ; "145")]
#[test_case(r#####"tw`outline-cyan-900`"#####, r#####"({
  outlineColor: "#164e63",
})
;"##### ; "146")]
#[test_case(r#####"tw`outline-sky-50`"#####, r#####"({
  outlineColor: "#f0f9ff",
})
;"##### ; "147")]
#[test_case(r#####"tw`outline-sky-100`"#####, r#####"({
  outlineColor: "#e0f2fe",
})
;"##### ; "148")]
#[test_case(r#####"tw`outline-sky-200`"#####, r#####"({
  outlineColor: "#bae6fd",
})
;"##### ; "149")]
#[test_case(r#####"tw`outline-sky-300`"#####, r#####"({
  outlineColor: "#7dd3fc",
})
;"##### ; "150")]
#[test_case(r#####"tw`outline-sky-400`"#####, r#####"({
  outlineColor: "#38bdf8",
})
;"##### ; "151")]
#[test_case(r#####"tw`outline-sky-500`"#####, r#####"({
  outlineColor: "#0ea5e9",
})
;"##### ; "152")]
#[test_case(r#####"tw`outline-sky-600`"#####, r#####"({
  outlineColor: "#0284c7",
})
;"##### ; "153")]
#[test_case(r#####"tw`outline-sky-700`"#####, r#####"({
  outlineColor: "#0369a1",
})
;"##### ; "154")]
#[test_case(r#####"tw`outline-sky-800`"#####, r#####"({
  outlineColor: "#075985",
})
;"##### ; "155")]
#[test_case(r#####"tw`outline-sky-900`"#####, r#####"({
  outlineColor: "#0c4a6e",
})
;"##### ; "156")]
#[test_case(r#####"tw`outline-blue-50`"#####, r#####"({
  outlineColor: "#eff6ff",
})
;"##### ; "157")]
#[test_case(r#####"tw`outline-blue-100`"#####, r#####"({
  outlineColor: "#dbeafe",
})
;"##### ; "158")]
#[test_case(r#####"tw`outline-blue-200`"#####, r#####"({
  outlineColor: "#bfdbfe",
})
;"##### ; "159")]
#[test_case(r#####"tw`outline-blue-300`"#####, r#####"({
  outlineColor: "#93c5fd",
})
;"##### ; "160")]
#[test_case(r#####"tw`outline-blue-400`"#####, r#####"({
  outlineColor: "#60a5fa",
})
;"##### ; "161")]
#[test_case(r#####"tw`outline-blue-500`"#####, r#####"({
  outlineColor: "#3b82f6",
})
;"##### ; "162")]
#[test_case(r#####"tw`outline-blue-600`"#####, r#####"({
  outlineColor: "#2563eb",
})
;"##### ; "163")]
#[test_case(r#####"tw`outline-blue-700`"#####, r#####"({
  outlineColor: "#1d4ed8",
})
;"##### ; "164")]
#[test_case(r#####"tw`outline-blue-800`"#####, r#####"({
  outlineColor: "#1e40af",
})
;"##### ; "165")]
#[test_case(r#####"tw`outline-blue-900`"#####, r#####"({
  outlineColor: "#1e3a8a",
})
;"##### ; "166")]
#[test_case(r#####"tw`outline-indigo-50`"#####, r#####"({
  outlineColor: "#eef2ff",
})
;"##### ; "167")]
#[test_case(r#####"tw`outline-indigo-100`"#####, r#####"({
  outlineColor: "#e0e7ff",
})
;"##### ; "168")]
#[test_case(r#####"tw`outline-indigo-200`"#####, r#####"({
  outlineColor: "#c7d2fe",
})
;"##### ; "169")]
#[test_case(r#####"tw`outline-indigo-300`"#####, r#####"({
  outlineColor: "#a5b4fc",
})
;"##### ; "170")]
#[test_case(r#####"tw`outline-indigo-400`"#####, r#####"({
  outlineColor: "#818cf8",
})
;"##### ; "171")]
#[test_case(r#####"tw`outline-indigo-500`"#####, r#####"({
  outlineColor: "#6366f1",
})
;"##### ; "172")]
#[test_case(r#####"tw`outline-indigo-600`"#####, r#####"({
  outlineColor: "#4f46e5",
})
;"##### ; "173")]
#[test_case(r#####"tw`outline-indigo-700`"#####, r#####"({
  outlineColor: "#4338ca",
})
;"##### ; "174")]
#[test_case(r#####"tw`outline-indigo-800`"#####, r#####"({
  outlineColor: "#3730a3",
})
;"##### ; "175")]
#[test_case(r#####"tw`outline-indigo-900`"#####, r#####"({
  outlineColor: "#312e81",
})
;"##### ; "176")]
#[test_case(r#####"tw`outline-violet-50`"#####, r#####"({
  outlineColor: "#f5f3ff",
})
;"##### ; "177")]
#[test_case(r#####"tw`outline-violet-100`"#####, r#####"({
  outlineColor: "#ede9fe",
})
;"##### ; "178")]
#[test_case(r#####"tw`outline-violet-200`"#####, r#####"({
  outlineColor: "#ddd6fe",
})
;"##### ; "179")]
#[test_case(r#####"tw`outline-violet-300`"#####, r#####"({
  outlineColor: "#c4b5fd",
})
;"##### ; "180")]
#[test_case(r#####"tw`outline-violet-400`"#####, r#####"({
  outlineColor: "#a78bfa",
})
;"##### ; "181")]
#[test_case(r#####"tw`outline-violet-500`"#####, r#####"({
  outlineColor: "#8b5cf6",
})
;"##### ; "182")]
#[test_case(r#####"tw`outline-violet-600`"#####, r#####"({
  outlineColor: "#7c3aed",
})
;"##### ; "183")]
#[test_case(r#####"tw`outline-violet-700`"#####, r#####"({
  outlineColor: "#6d28d9",
})
;"##### ; "184")]
#[test_case(r#####"tw`outline-violet-800`"#####, r#####"({
  outlineColor: "#5b21b6",
})
;"##### ; "185")]
#[test_case(r#####"tw`outline-violet-900`"#####, r#####"({
  outlineColor: "#4c1d95",
})
;"##### ; "186")]
#[test_case(r#####"tw`outline-purple-50`"#####, r#####"({
  outlineColor: "#faf5ff",
})
;"##### ; "187")]
#[test_case(r#####"tw`outline-purple-100`"#####, r#####"({
  outlineColor: "#f3e8ff",
})
;"##### ; "188")]
#[test_case(r#####"tw`outline-purple-200`"#####, r#####"({
  outlineColor: "#e9d5ff",
})
;"##### ; "189")]
#[test_case(r#####"tw`outline-purple-300`"#####, r#####"({
  outlineColor: "#d8b4fe",
})
;"##### ; "190")]
#[test_case(r#####"tw`outline-purple-400`"#####, r#####"({
  outlineColor: "#c084fc",
})
;"##### ; "191")]
#[test_case(r#####"tw`outline-purple-500`"#####, r#####"({
  outlineColor: "#a855f7",
})
;"##### ; "192")]
#[test_case(r#####"tw`outline-purple-600`"#####, r#####"({
  outlineColor: "#9333ea",
})
;"##### ; "193")]
#[test_case(r#####"tw`outline-purple-700`"#####, r#####"({
  outlineColor: "#7e22ce",
})
;"##### ; "194")]
#[test_case(r#####"tw`outline-purple-800`"#####, r#####"({
  outlineColor: "#6b21a8",
})
;"##### ; "195")]
#[test_case(r#####"tw`outline-purple-900`"#####, r#####"({
  outlineColor: "#581c87",
})
;"##### ; "196")]
#[test_case(r#####"tw`outline-fuchsia-50`"#####, r#####"({
  outlineColor: "#fdf4ff",
})
;"##### ; "197")]
#[test_case(r#####"tw`outline-fuchsia-100`"#####, r#####"({
  outlineColor: "#fae8ff",
})
;"##### ; "198")]
#[test_case(r#####"tw`outline-fuchsia-200`"#####, r#####"({
  outlineColor: "#f5d0fe",
})
;"##### ; "199")]
#[test_case(r#####"tw`outline-fuchsia-300`"#####, r#####"({
  outlineColor: "#f0abfc",
})
;"##### ; "200")]
#[test_case(r#####"tw`outline-fuchsia-400`"#####, r#####"({
  outlineColor: "#e879f9",
})
;"##### ; "201")]
#[test_case(r#####"tw`outline-fuchsia-500`"#####, r#####"({
  outlineColor: "#d946ef",
})
;"##### ; "202")]
#[test_case(r#####"tw`outline-fuchsia-600`"#####, r#####"({
  outlineColor: "#c026d3",
})
;"##### ; "203")]
#[test_case(r#####"tw`outline-fuchsia-700`"#####, r#####"({
  outlineColor: "#a21caf",
})
;"##### ; "204")]
#[test_case(r#####"tw`outline-fuchsia-800`"#####, r#####"({
  outlineColor: "#86198f",
})
;"##### ; "205")]
#[test_case(r#####"tw`outline-fuchsia-900`"#####, r#####"({
  outlineColor: "#701a75",
})
;"##### ; "206")]
#[test_case(r#####"tw`outline-pink-50`"#####, r#####"({
  outlineColor: "#fdf2f8",
})
;"##### ; "207")]
#[test_case(r#####"tw`outline-pink-100`"#####, r#####"({
  outlineColor: "#fce7f3",
})
;"##### ; "208")]
#[test_case(r#####"tw`outline-pink-200`"#####, r#####"({
  outlineColor: "#fbcfe8",
})
;"##### ; "209")]
#[test_case(r#####"tw`outline-pink-300`"#####, r#####"({
  outlineColor: "#f9a8d4",
})
;"##### ; "210")]
#[test_case(r#####"tw`outline-pink-400`"#####, r#####"({
  outlineColor: "#f472b6",
})
;"##### ; "211")]
#[test_case(r#####"tw`outline-pink-500`"#####, r#####"({
  outlineColor: "#ec4899",
})
;"##### ; "212")]
#[test_case(r#####"tw`outline-pink-600`"#####, r#####"({
  outlineColor: "#db2777",
})
;"##### ; "213")]
#[test_case(r#####"tw`outline-pink-700`"#####, r#####"({
  outlineColor: "#be185d",
})
;"##### ; "214")]
#[test_case(r#####"tw`outline-pink-800`"#####, r#####"({
  outlineColor: "#9d174d",
})
;"##### ; "215")]
#[test_case(r#####"tw`outline-pink-900`"#####, r#####"({
  outlineColor: "#831843",
})
;"##### ; "216")]
#[test_case(r#####"tw`outline-rose-50`"#####, r#####"({
  outlineColor: "#fff1f2",
})
;"##### ; "217")]
#[test_case(r#####"tw`outline-rose-100`"#####, r#####"({
  outlineColor: "#ffe4e6",
})
;"##### ; "218")]
#[test_case(r#####"tw`outline-rose-200`"#####, r#####"({
  outlineColor: "#fecdd3",
})
;"##### ; "219")]
#[test_case(r#####"tw`outline-rose-300`"#####, r#####"({
  outlineColor: "#fda4af",
})
;"##### ; "220")]
#[test_case(r#####"tw`outline-rose-400`"#####, r#####"({
  outlineColor: "#fb7185",
})
;"##### ; "221")]
#[test_case(r#####"tw`outline-rose-500`"#####, r#####"({
  outlineColor: "#f43f5e",
})
;"##### ; "222")]
#[test_case(r#####"tw`outline-rose-600`"#####, r#####"({
  outlineColor: "#e11d48",
})
;"##### ; "223")]
#[test_case(r#####"tw`outline-rose-700`"#####, r#####"({
  outlineColor: "#be123c",
})
;"##### ; "224")]
#[test_case(r#####"tw`outline-rose-800`"#####, r#####"({
  outlineColor: "#9f1239",
})
;"##### ; "225")]
#[test_case(r#####"tw`outline-rose-900`"#####, r#####"({
  outlineColor: "#881337",
})
;"##### ; "226")]
#[test_case(r#####"tw`outline-rose-900/50`"#####, r#####"({
  outlineColor: "rgb(136 19 55 / 0.5)",
})
;"##### ; "227")]
#[test_case(r#####"tw`outline-rose-900/[.50]`"#####, r#####"({
  outlineColor: "rgb(136 19 55 / .50)",
})
;"##### ; "228")]
#[test_case(r#####"tw`outline-[#243c5a]`"#####, r#####"({
  outlineColor: "#243c5a",
})
;"##### ; "229")]
#[test_case(r#####"tw`outline-[length:500px]`"#####, r#####"({
  outlineWidth: "500px",
})
;"##### ; "230")]
#[test_case(r#####"tw`outline-[color:red]`"#####, r#####"({
  outlineColor: "red",
})
;"##### ; "231")]
#[test_case(r#####"tw`outline-[percentage:10%]`"#####, r#####"({
  outlineWidth: "10%",
}) // tw`outline-[var(--outline)]` // Unsupported by tailwindcss

;"##### ; "232")]
#[test_case(r#####"tw`outline-[color:var(--outline)]`"#####, r#####"({
  outlineColor: "var(--outline)",
})
;"##### ; "233")]
#[test_case(r#####"tw`outline-[length:var(--outline)]`"#####, r#####"({
  outlineWidth: "var(--outline)",
})
;"##### ; "234")]
#[test_case(r#####"tw`outline outline-dashed outline-none outline-black outline-4 outline-offset-2`"#####, r#####"({
  outline: "2px solid transparent",
  outlineOffset: "2px",
  outlineStyle: "dashed",
  outlineWidth: "4px",
  outlineColor: "#000",
})"##### ; "235")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
