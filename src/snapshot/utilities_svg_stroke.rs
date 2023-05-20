use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`stroke`"#####, r#####"({
  none: "none",
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
#[test_case(r#####"tw`stroke-none`"#####, r#####"({
  stroke: "none",
})
;"##### ; "2")]
#[test_case(r#####"tw`stroke-inherit`"#####, r#####"({
  stroke: "inherit",
})
;"##### ; "3")]
#[test_case(r#####"tw`stroke-current`"#####, r#####"({
  stroke: "currentColor",
})
;"##### ; "4")]
#[test_case(r#####"tw`stroke-transparent`"#####, r#####"({
  stroke: "transparent",
})
;"##### ; "5")]
#[test_case(r#####"tw`stroke-black`"#####, r#####"({
  stroke: "#000",
})
;"##### ; "6")]
#[test_case(r#####"tw`stroke-white`"#####, r#####"({
  stroke: "#fff",
})
;"##### ; "7")]
#[test_case(r#####"tw`stroke-slate-50`"#####, r#####"({
  stroke: "#f8fafc",
})
;"##### ; "8")]
#[test_case(r#####"tw`stroke-slate-100`"#####, r#####"({
  stroke: "#f1f5f9",
})
;"##### ; "9")]
#[test_case(r#####"tw`stroke-slate-200`"#####, r#####"({
  stroke: "#e2e8f0",
})
;"##### ; "10")]
#[test_case(r#####"tw`stroke-slate-300`"#####, r#####"({
  stroke: "#cbd5e1",
})
;"##### ; "11")]
#[test_case(r#####"tw`stroke-slate-400`"#####, r#####"({
  stroke: "#94a3b8",
})
;"##### ; "12")]
#[test_case(r#####"tw`stroke-slate-500`"#####, r#####"({
  stroke: "#64748b",
})
;"##### ; "13")]
#[test_case(r#####"tw`stroke-slate-600`"#####, r#####"({
  stroke: "#475569",
})
;"##### ; "14")]
#[test_case(r#####"tw`stroke-slate-700`"#####, r#####"({
  stroke: "#334155",
})
;"##### ; "15")]
#[test_case(r#####"tw`stroke-slate-800`"#####, r#####"({
  stroke: "#1e293b",
})
;"##### ; "16")]
#[test_case(r#####"tw`stroke-slate-900`"#####, r#####"({
  stroke: "#0f172a",
})
;"##### ; "17")]
#[test_case(r#####"tw`stroke-gray-50`"#####, r#####"({
  stroke: "#f9fafb",
})
;"##### ; "18")]
#[test_case(r#####"tw`stroke-gray-100`"#####, r#####"({
  stroke: "#f3f4f6",
})
;"##### ; "19")]
#[test_case(r#####"tw`stroke-gray-200`"#####, r#####"({
  stroke: "#e5e7eb",
})
;"##### ; "20")]
#[test_case(r#####"tw`stroke-gray-300`"#####, r#####"({
  stroke: "#d1d5db",
})
;"##### ; "21")]
#[test_case(r#####"tw`stroke-gray-400`"#####, r#####"({
  stroke: "#9ca3af",
})
;"##### ; "22")]
#[test_case(r#####"tw`stroke-gray-500`"#####, r#####"({
  stroke: "#6b7280",
})
;"##### ; "23")]
#[test_case(r#####"tw`stroke-gray-600`"#####, r#####"({
  stroke: "#4b5563",
})
;"##### ; "24")]
#[test_case(r#####"tw`stroke-gray-700`"#####, r#####"({
  stroke: "#374151",
})
;"##### ; "25")]
#[test_case(r#####"tw`stroke-gray-800`"#####, r#####"({
  stroke: "#1f2937",
})
;"##### ; "26")]
#[test_case(r#####"tw`stroke-gray-900`"#####, r#####"({
  stroke: "#111827",
})
;"##### ; "27")]
#[test_case(r#####"tw`stroke-zinc-50`"#####, r#####"({
  stroke: "#fafafa",
})
;"##### ; "28")]
#[test_case(r#####"tw`stroke-zinc-100`"#####, r#####"({
  stroke: "#f4f4f5",
})
;"##### ; "29")]
#[test_case(r#####"tw`stroke-zinc-200`"#####, r#####"({
  stroke: "#e4e4e7",
})
;"##### ; "30")]
#[test_case(r#####"tw`stroke-zinc-300`"#####, r#####"({
  stroke: "#d4d4d8",
})
;"##### ; "31")]
#[test_case(r#####"tw`stroke-zinc-400`"#####, r#####"({
  stroke: "#a1a1aa",
})
;"##### ; "32")]
#[test_case(r#####"tw`stroke-zinc-500`"#####, r#####"({
  stroke: "#71717a",
})
;"##### ; "33")]
#[test_case(r#####"tw`stroke-zinc-600`"#####, r#####"({
  stroke: "#52525b",
})
;"##### ; "34")]
#[test_case(r#####"tw`stroke-zinc-700`"#####, r#####"({
  stroke: "#3f3f46",
})
;"##### ; "35")]
#[test_case(r#####"tw`stroke-zinc-800`"#####, r#####"({
  stroke: "#27272a",
})
;"##### ; "36")]
#[test_case(r#####"tw`stroke-zinc-900`"#####, r#####"({
  stroke: "#18181b",
})
;"##### ; "37")]
#[test_case(r#####"tw`stroke-neutral-50`"#####, r#####"({
  stroke: "#fafafa",
})
;"##### ; "38")]
#[test_case(r#####"tw`stroke-neutral-100`"#####, r#####"({
  stroke: "#f5f5f5",
})
;"##### ; "39")]
#[test_case(r#####"tw`stroke-neutral-200`"#####, r#####"({
  stroke: "#e5e5e5",
})
;"##### ; "40")]
#[test_case(r#####"tw`stroke-neutral-300`"#####, r#####"({
  stroke: "#d4d4d4",
})
;"##### ; "41")]
#[test_case(r#####"tw`stroke-neutral-400`"#####, r#####"({
  stroke: "#a3a3a3",
})
;"##### ; "42")]
#[test_case(r#####"tw`stroke-neutral-500`"#####, r#####"({
  stroke: "#737373",
})
;"##### ; "43")]
#[test_case(r#####"tw`stroke-neutral-600`"#####, r#####"({
  stroke: "#525252",
})
;"##### ; "44")]
#[test_case(r#####"tw`stroke-neutral-700`"#####, r#####"({
  stroke: "#404040",
})
;"##### ; "45")]
#[test_case(r#####"tw`stroke-neutral-800`"#####, r#####"({
  stroke: "#262626",
})
;"##### ; "46")]
#[test_case(r#####"tw`stroke-neutral-900`"#####, r#####"({
  stroke: "#171717",
})
;"##### ; "47")]
#[test_case(r#####"tw`stroke-stone-50`"#####, r#####"({
  stroke: "#fafaf9",
})
;"##### ; "48")]
#[test_case(r#####"tw`stroke-stone-100`"#####, r#####"({
  stroke: "#f5f5f4",
})
;"##### ; "49")]
#[test_case(r#####"tw`stroke-stone-200`"#####, r#####"({
  stroke: "#e7e5e4",
})
;"##### ; "50")]
#[test_case(r#####"tw`stroke-stone-300`"#####, r#####"({
  stroke: "#d6d3d1",
})
;"##### ; "51")]
#[test_case(r#####"tw`stroke-stone-400`"#####, r#####"({
  stroke: "#a8a29e",
})
;"##### ; "52")]
#[test_case(r#####"tw`stroke-stone-500`"#####, r#####"({
  stroke: "#78716c",
})
;"##### ; "53")]
#[test_case(r#####"tw`stroke-stone-600`"#####, r#####"({
  stroke: "#57534e",
})
;"##### ; "54")]
#[test_case(r#####"tw`stroke-stone-700`"#####, r#####"({
  stroke: "#44403c",
})
;"##### ; "55")]
#[test_case(r#####"tw`stroke-stone-800`"#####, r#####"({
  stroke: "#292524",
})
;"##### ; "56")]
#[test_case(r#####"tw`stroke-stone-900`"#####, r#####"({
  stroke: "#1c1917",
})
;"##### ; "57")]
#[test_case(r#####"tw`stroke-red-50`"#####, r#####"({
  stroke: "#fef2f2",
})
;"##### ; "58")]
#[test_case(r#####"tw`stroke-red-100`"#####, r#####"({
  stroke: "#fee2e2",
})
;"##### ; "59")]
#[test_case(r#####"tw`stroke-red-200`"#####, r#####"({
  stroke: "#fecaca",
})
;"##### ; "60")]
#[test_case(r#####"tw`stroke-red-300`"#####, r#####"({
  stroke: "#fca5a5",
})
;"##### ; "61")]
#[test_case(r#####"tw`stroke-red-400`"#####, r#####"({
  stroke: "#f87171",
})
;"##### ; "62")]
#[test_case(r#####"tw`stroke-red-500`"#####, r#####"({
  stroke: "#ef4444",
})
;"##### ; "63")]
#[test_case(r#####"tw`stroke-red-600`"#####, r#####"({
  stroke: "#dc2626",
})
;"##### ; "64")]
#[test_case(r#####"tw`stroke-red-700`"#####, r#####"({
  stroke: "#b91c1c",
})
;"##### ; "65")]
#[test_case(r#####"tw`stroke-red-800`"#####, r#####"({
  stroke: "#991b1b",
})
;"##### ; "66")]
#[test_case(r#####"tw`stroke-red-900`"#####, r#####"({
  stroke: "#7f1d1d",
})
;"##### ; "67")]
#[test_case(r#####"tw`stroke-orange-50`"#####, r#####"({
  stroke: "#fff7ed",
})
;"##### ; "68")]
#[test_case(r#####"tw`stroke-orange-100`"#####, r#####"({
  stroke: "#ffedd5",
})
;"##### ; "69")]
#[test_case(r#####"tw`stroke-orange-200`"#####, r#####"({
  stroke: "#fed7aa",
})
;"##### ; "70")]
#[test_case(r#####"tw`stroke-orange-300`"#####, r#####"({
  stroke: "#fdba74",
})
;"##### ; "71")]
#[test_case(r#####"tw`stroke-orange-400`"#####, r#####"({
  stroke: "#fb923c",
})
;"##### ; "72")]
#[test_case(r#####"tw`stroke-orange-500`"#####, r#####"({
  stroke: "#f97316",
})
;"##### ; "73")]
#[test_case(r#####"tw`stroke-orange-600`"#####, r#####"({
  stroke: "#ea580c",
})
;"##### ; "74")]
#[test_case(r#####"tw`stroke-orange-700`"#####, r#####"({
  stroke: "#c2410c",
})
;"##### ; "75")]
#[test_case(r#####"tw`stroke-orange-800`"#####, r#####"({
  stroke: "#9a3412",
})
;"##### ; "76")]
#[test_case(r#####"tw`stroke-orange-900`"#####, r#####"({
  stroke: "#7c2d12",
})
;"##### ; "77")]
#[test_case(r#####"tw`stroke-amber-50`"#####, r#####"({
  stroke: "#fffbeb",
})
;"##### ; "78")]
#[test_case(r#####"tw`stroke-amber-100`"#####, r#####"({
  stroke: "#fef3c7",
})
;"##### ; "79")]
#[test_case(r#####"tw`stroke-amber-200`"#####, r#####"({
  stroke: "#fde68a",
})
;"##### ; "80")]
#[test_case(r#####"tw`stroke-amber-300`"#####, r#####"({
  stroke: "#fcd34d",
})
;"##### ; "81")]
#[test_case(r#####"tw`stroke-amber-400`"#####, r#####"({
  stroke: "#fbbf24",
})
;"##### ; "82")]
#[test_case(r#####"tw`stroke-amber-500`"#####, r#####"({
  stroke: "#f59e0b",
})
;"##### ; "83")]
#[test_case(r#####"tw`stroke-amber-600`"#####, r#####"({
  stroke: "#d97706",
})
;"##### ; "84")]
#[test_case(r#####"tw`stroke-amber-700`"#####, r#####"({
  stroke: "#b45309",
})
;"##### ; "85")]
#[test_case(r#####"tw`stroke-amber-800`"#####, r#####"({
  stroke: "#92400e",
})
;"##### ; "86")]
#[test_case(r#####"tw`stroke-amber-900`"#####, r#####"({
  stroke: "#78350f",
})
;"##### ; "87")]
#[test_case(r#####"tw`stroke-yellow-50`"#####, r#####"({
  stroke: "#fefce8",
})
;"##### ; "88")]
#[test_case(r#####"tw`stroke-yellow-100`"#####, r#####"({
  stroke: "#fef9c3",
})
;"##### ; "89")]
#[test_case(r#####"tw`stroke-yellow-200`"#####, r#####"({
  stroke: "#fef08a",
})
;"##### ; "90")]
#[test_case(r#####"tw`stroke-yellow-300`"#####, r#####"({
  stroke: "#fde047",
})
;"##### ; "91")]
#[test_case(r#####"tw`stroke-yellow-400`"#####, r#####"({
  stroke: "#facc15",
})
;"##### ; "92")]
#[test_case(r#####"tw`stroke-yellow-500`"#####, r#####"({
  stroke: "#eab308",
})
;"##### ; "93")]
#[test_case(r#####"tw`stroke-yellow-600`"#####, r#####"({
  stroke: "#ca8a04",
})
;"##### ; "94")]
#[test_case(r#####"tw`stroke-yellow-700`"#####, r#####"({
  stroke: "#a16207",
})
;"##### ; "95")]
#[test_case(r#####"tw`stroke-yellow-800`"#####, r#####"({
  stroke: "#854d0e",
})
;"##### ; "96")]
#[test_case(r#####"tw`stroke-yellow-900`"#####, r#####"({
  stroke: "#713f12",
})
;"##### ; "97")]
#[test_case(r#####"tw`stroke-lime-50`"#####, r#####"({
  stroke: "#f7fee7",
})
;"##### ; "98")]
#[test_case(r#####"tw`stroke-lime-100`"#####, r#####"({
  stroke: "#ecfccb",
})
;"##### ; "99")]
#[test_case(r#####"tw`stroke-lime-200`"#####, r#####"({
  stroke: "#d9f99d",
})
;"##### ; "100")]
#[test_case(r#####"tw`stroke-lime-300`"#####, r#####"({
  stroke: "#bef264",
})
;"##### ; "101")]
#[test_case(r#####"tw`stroke-lime-400`"#####, r#####"({
  stroke: "#a3e635",
})
;"##### ; "102")]
#[test_case(r#####"tw`stroke-lime-500`"#####, r#####"({
  stroke: "#84cc16",
})
;"##### ; "103")]
#[test_case(r#####"tw`stroke-lime-600`"#####, r#####"({
  stroke: "#65a30d",
})
;"##### ; "104")]
#[test_case(r#####"tw`stroke-lime-700`"#####, r#####"({
  stroke: "#4d7c0f",
})
;"##### ; "105")]
#[test_case(r#####"tw`stroke-lime-800`"#####, r#####"({
  stroke: "#3f6212",
})
;"##### ; "106")]
#[test_case(r#####"tw`stroke-lime-900`"#####, r#####"({
  stroke: "#365314",
})
;"##### ; "107")]
#[test_case(r#####"tw`stroke-green-50`"#####, r#####"({
  stroke: "#f0fdf4",
})
;"##### ; "108")]
#[test_case(r#####"tw`stroke-green-100`"#####, r#####"({
  stroke: "#dcfce7",
})
;"##### ; "109")]
#[test_case(r#####"tw`stroke-green-200`"#####, r#####"({
  stroke: "#bbf7d0",
})
;"##### ; "110")]
#[test_case(r#####"tw`stroke-green-300`"#####, r#####"({
  stroke: "#86efac",
})
;"##### ; "111")]
#[test_case(r#####"tw`stroke-green-400`"#####, r#####"({
  stroke: "#4ade80",
})
;"##### ; "112")]
#[test_case(r#####"tw`stroke-green-500`"#####, r#####"({
  stroke: "#22c55e",
})
;"##### ; "113")]
#[test_case(r#####"tw`stroke-green-600`"#####, r#####"({
  stroke: "#16a34a",
})
;"##### ; "114")]
#[test_case(r#####"tw`stroke-green-700`"#####, r#####"({
  stroke: "#15803d",
})
;"##### ; "115")]
#[test_case(r#####"tw`stroke-green-800`"#####, r#####"({
  stroke: "#166534",
})
;"##### ; "116")]
#[test_case(r#####"tw`stroke-green-900`"#####, r#####"({
  stroke: "#14532d",
})
;"##### ; "117")]
#[test_case(r#####"tw`stroke-emerald-50`"#####, r#####"({
  stroke: "#ecfdf5",
})
;"##### ; "118")]
#[test_case(r#####"tw`stroke-emerald-100`"#####, r#####"({
  stroke: "#d1fae5",
})
;"##### ; "119")]
#[test_case(r#####"tw`stroke-emerald-200`"#####, r#####"({
  stroke: "#a7f3d0",
})
;"##### ; "120")]
#[test_case(r#####"tw`stroke-emerald-300`"#####, r#####"({
  stroke: "#6ee7b7",
})
;"##### ; "121")]
#[test_case(r#####"tw`stroke-emerald-400`"#####, r#####"({
  stroke: "#34d399",
})
;"##### ; "122")]
#[test_case(r#####"tw`stroke-emerald-500`"#####, r#####"({
  stroke: "#10b981",
})
;"##### ; "123")]
#[test_case(r#####"tw`stroke-emerald-600`"#####, r#####"({
  stroke: "#059669",
})
;"##### ; "124")]
#[test_case(r#####"tw`stroke-emerald-700`"#####, r#####"({
  stroke: "#047857",
})
;"##### ; "125")]
#[test_case(r#####"tw`stroke-emerald-800`"#####, r#####"({
  stroke: "#065f46",
})
;"##### ; "126")]
#[test_case(r#####"tw`stroke-emerald-900`"#####, r#####"({
  stroke: "#064e3b",
})
;"##### ; "127")]
#[test_case(r#####"tw`stroke-teal-50`"#####, r#####"({
  stroke: "#f0fdfa",
})
;"##### ; "128")]
#[test_case(r#####"tw`stroke-teal-100`"#####, r#####"({
  stroke: "#ccfbf1",
})
;"##### ; "129")]
#[test_case(r#####"tw`stroke-teal-200`"#####, r#####"({
  stroke: "#99f6e4",
})
;"##### ; "130")]
#[test_case(r#####"tw`stroke-teal-300`"#####, r#####"({
  stroke: "#5eead4",
})
;"##### ; "131")]
#[test_case(r#####"tw`stroke-teal-400`"#####, r#####"({
  stroke: "#2dd4bf",
})
;"##### ; "132")]
#[test_case(r#####"tw`stroke-teal-500`"#####, r#####"({
  stroke: "#14b8a6",
})
;"##### ; "133")]
#[test_case(r#####"tw`stroke-teal-600`"#####, r#####"({
  stroke: "#0d9488",
})
;"##### ; "134")]
#[test_case(r#####"tw`stroke-teal-700`"#####, r#####"({
  stroke: "#0f766e",
})
;"##### ; "135")]
#[test_case(r#####"tw`stroke-teal-800`"#####, r#####"({
  stroke: "#115e59",
})
;"##### ; "136")]
#[test_case(r#####"tw`stroke-teal-900`"#####, r#####"({
  stroke: "#134e4a",
})
;"##### ; "137")]
#[test_case(r#####"tw`stroke-cyan-50`"#####, r#####"({
  stroke: "#ecfeff",
})
;"##### ; "138")]
#[test_case(r#####"tw`stroke-cyan-100`"#####, r#####"({
  stroke: "#cffafe",
})
;"##### ; "139")]
#[test_case(r#####"tw`stroke-cyan-200`"#####, r#####"({
  stroke: "#a5f3fc",
})
;"##### ; "140")]
#[test_case(r#####"tw`stroke-cyan-300`"#####, r#####"({
  stroke: "#67e8f9",
})
;"##### ; "141")]
#[test_case(r#####"tw`stroke-cyan-400`"#####, r#####"({
  stroke: "#22d3ee",
})
;"##### ; "142")]
#[test_case(r#####"tw`stroke-cyan-500`"#####, r#####"({
  stroke: "#06b6d4",
})
;"##### ; "143")]
#[test_case(r#####"tw`stroke-cyan-600`"#####, r#####"({
  stroke: "#0891b2",
})
;"##### ; "144")]
#[test_case(r#####"tw`stroke-cyan-700`"#####, r#####"({
  stroke: "#0e7490",
})
;"##### ; "145")]
#[test_case(r#####"tw`stroke-cyan-800`"#####, r#####"({
  stroke: "#155e75",
})
;"##### ; "146")]
#[test_case(r#####"tw`stroke-cyan-900`"#####, r#####"({
  stroke: "#164e63",
})
;"##### ; "147")]
#[test_case(r#####"tw`stroke-sky-50`"#####, r#####"({
  stroke: "#f0f9ff",
})
;"##### ; "148")]
#[test_case(r#####"tw`stroke-sky-100`"#####, r#####"({
  stroke: "#e0f2fe",
})
;"##### ; "149")]
#[test_case(r#####"tw`stroke-sky-200`"#####, r#####"({
  stroke: "#bae6fd",
})
;"##### ; "150")]
#[test_case(r#####"tw`stroke-sky-300`"#####, r#####"({
  stroke: "#7dd3fc",
})
;"##### ; "151")]
#[test_case(r#####"tw`stroke-sky-400`"#####, r#####"({
  stroke: "#38bdf8",
})
;"##### ; "152")]
#[test_case(r#####"tw`stroke-sky-500`"#####, r#####"({
  stroke: "#0ea5e9",
})
;"##### ; "153")]
#[test_case(r#####"tw`stroke-sky-600`"#####, r#####"({
  stroke: "#0284c7",
})
;"##### ; "154")]
#[test_case(r#####"tw`stroke-sky-700`"#####, r#####"({
  stroke: "#0369a1",
})
;"##### ; "155")]
#[test_case(r#####"tw`stroke-sky-800`"#####, r#####"({
  stroke: "#075985",
})
;"##### ; "156")]
#[test_case(r#####"tw`stroke-sky-900`"#####, r#####"({
  stroke: "#0c4a6e",
})
;"##### ; "157")]
#[test_case(r#####"tw`stroke-blue-50`"#####, r#####"({
  stroke: "#eff6ff",
})
;"##### ; "158")]
#[test_case(r#####"tw`stroke-blue-100`"#####, r#####"({
  stroke: "#dbeafe",
})
;"##### ; "159")]
#[test_case(r#####"tw`stroke-blue-200`"#####, r#####"({
  stroke: "#bfdbfe",
})
;"##### ; "160")]
#[test_case(r#####"tw`stroke-blue-300`"#####, r#####"({
  stroke: "#93c5fd",
})
;"##### ; "161")]
#[test_case(r#####"tw`stroke-blue-400`"#####, r#####"({
  stroke: "#60a5fa",
})
;"##### ; "162")]
#[test_case(r#####"tw`stroke-blue-500`"#####, r#####"({
  stroke: "#3b82f6",
})
;"##### ; "163")]
#[test_case(r#####"tw`stroke-blue-600`"#####, r#####"({
  stroke: "#2563eb",
})
;"##### ; "164")]
#[test_case(r#####"tw`stroke-blue-700`"#####, r#####"({
  stroke: "#1d4ed8",
})
;"##### ; "165")]
#[test_case(r#####"tw`stroke-blue-800`"#####, r#####"({
  stroke: "#1e40af",
})
;"##### ; "166")]
#[test_case(r#####"tw`stroke-blue-900`"#####, r#####"({
  stroke: "#1e3a8a",
})
;"##### ; "167")]
#[test_case(r#####"tw`stroke-indigo-50`"#####, r#####"({
  stroke: "#eef2ff",
})
;"##### ; "168")]
#[test_case(r#####"tw`stroke-indigo-100`"#####, r#####"({
  stroke: "#e0e7ff",
})
;"##### ; "169")]
#[test_case(r#####"tw`stroke-indigo-200`"#####, r#####"({
  stroke: "#c7d2fe",
})
;"##### ; "170")]
#[test_case(r#####"tw`stroke-indigo-300`"#####, r#####"({
  stroke: "#a5b4fc",
})
;"##### ; "171")]
#[test_case(r#####"tw`stroke-indigo-400`"#####, r#####"({
  stroke: "#818cf8",
})
;"##### ; "172")]
#[test_case(r#####"tw`stroke-indigo-500`"#####, r#####"({
  stroke: "#6366f1",
})
;"##### ; "173")]
#[test_case(r#####"tw`stroke-indigo-600`"#####, r#####"({
  stroke: "#4f46e5",
})
;"##### ; "174")]
#[test_case(r#####"tw`stroke-indigo-700`"#####, r#####"({
  stroke: "#4338ca",
})
;"##### ; "175")]
#[test_case(r#####"tw`stroke-indigo-800`"#####, r#####"({
  stroke: "#3730a3",
})
;"##### ; "176")]
#[test_case(r#####"tw`stroke-indigo-900`"#####, r#####"({
  stroke: "#312e81",
})
;"##### ; "177")]
#[test_case(r#####"tw`stroke-violet-50`"#####, r#####"({
  stroke: "#f5f3ff",
})
;"##### ; "178")]
#[test_case(r#####"tw`stroke-violet-100`"#####, r#####"({
  stroke: "#ede9fe",
})
;"##### ; "179")]
#[test_case(r#####"tw`stroke-violet-200`"#####, r#####"({
  stroke: "#ddd6fe",
})
;"##### ; "180")]
#[test_case(r#####"tw`stroke-violet-300`"#####, r#####"({
  stroke: "#c4b5fd",
})
;"##### ; "181")]
#[test_case(r#####"tw`stroke-violet-400`"#####, r#####"({
  stroke: "#a78bfa",
})
;"##### ; "182")]
#[test_case(r#####"tw`stroke-violet-500`"#####, r#####"({
  stroke: "#8b5cf6",
})
;"##### ; "183")]
#[test_case(r#####"tw`stroke-violet-600`"#####, r#####"({
  stroke: "#7c3aed",
})
;"##### ; "184")]
#[test_case(r#####"tw`stroke-violet-700`"#####, r#####"({
  stroke: "#6d28d9",
})
;"##### ; "185")]
#[test_case(r#####"tw`stroke-violet-800`"#####, r#####"({
  stroke: "#5b21b6",
})
;"##### ; "186")]
#[test_case(r#####"tw`stroke-violet-900`"#####, r#####"({
  stroke: "#4c1d95",
})
;"##### ; "187")]
#[test_case(r#####"tw`stroke-purple-50`"#####, r#####"({
  stroke: "#faf5ff",
})
;"##### ; "188")]
#[test_case(r#####"tw`stroke-purple-100`"#####, r#####"({
  stroke: "#f3e8ff",
})
;"##### ; "189")]
#[test_case(r#####"tw`stroke-purple-200`"#####, r#####"({
  stroke: "#e9d5ff",
})
;"##### ; "190")]
#[test_case(r#####"tw`stroke-purple-300`"#####, r#####"({
  stroke: "#d8b4fe",
})
;"##### ; "191")]
#[test_case(r#####"tw`stroke-purple-400`"#####, r#####"({
  stroke: "#c084fc",
})
;"##### ; "192")]
#[test_case(r#####"tw`stroke-purple-500`"#####, r#####"({
  stroke: "#a855f7",
})
;"##### ; "193")]
#[test_case(r#####"tw`stroke-purple-600`"#####, r#####"({
  stroke: "#9333ea",
})
;"##### ; "194")]
#[test_case(r#####"tw`stroke-purple-700`"#####, r#####"({
  stroke: "#7e22ce",
})
;"##### ; "195")]
#[test_case(r#####"tw`stroke-purple-800`"#####, r#####"({
  stroke: "#6b21a8",
})
;"##### ; "196")]
#[test_case(r#####"tw`stroke-purple-900`"#####, r#####"({
  stroke: "#581c87",
})
;"##### ; "197")]
#[test_case(r#####"tw`stroke-fuchsia-50`"#####, r#####"({
  stroke: "#fdf4ff",
})
;"##### ; "198")]
#[test_case(r#####"tw`stroke-fuchsia-100`"#####, r#####"({
  stroke: "#fae8ff",
})
;"##### ; "199")]
#[test_case(r#####"tw`stroke-fuchsia-200`"#####, r#####"({
  stroke: "#f5d0fe",
})
;"##### ; "200")]
#[test_case(r#####"tw`stroke-fuchsia-300`"#####, r#####"({
  stroke: "#f0abfc",
})
;"##### ; "201")]
#[test_case(r#####"tw`stroke-fuchsia-400`"#####, r#####"({
  stroke: "#e879f9",
})
;"##### ; "202")]
#[test_case(r#####"tw`stroke-fuchsia-500`"#####, r#####"({
  stroke: "#d946ef",
})
;"##### ; "203")]
#[test_case(r#####"tw`stroke-fuchsia-600`"#####, r#####"({
  stroke: "#c026d3",
})
;"##### ; "204")]
#[test_case(r#####"tw`stroke-fuchsia-700`"#####, r#####"({
  stroke: "#a21caf",
})
;"##### ; "205")]
#[test_case(r#####"tw`stroke-fuchsia-800`"#####, r#####"({
  stroke: "#86198f",
})
;"##### ; "206")]
#[test_case(r#####"tw`stroke-fuchsia-900`"#####, r#####"({
  stroke: "#701a75",
})
;"##### ; "207")]
#[test_case(r#####"tw`stroke-pink-50`"#####, r#####"({
  stroke: "#fdf2f8",
})
;"##### ; "208")]
#[test_case(r#####"tw`stroke-pink-100`"#####, r#####"({
  stroke: "#fce7f3",
})
;"##### ; "209")]
#[test_case(r#####"tw`stroke-pink-200`"#####, r#####"({
  stroke: "#fbcfe8",
})
;"##### ; "210")]
#[test_case(r#####"tw`stroke-pink-300`"#####, r#####"({
  stroke: "#f9a8d4",
})
;"##### ; "211")]
#[test_case(r#####"tw`stroke-pink-400`"#####, r#####"({
  stroke: "#f472b6",
})
;"##### ; "212")]
#[test_case(r#####"tw`stroke-pink-500`"#####, r#####"({
  stroke: "#ec4899",
})
;"##### ; "213")]
#[test_case(r#####"tw`stroke-pink-600`"#####, r#####"({
  stroke: "#db2777",
})
;"##### ; "214")]
#[test_case(r#####"tw`stroke-pink-700`"#####, r#####"({
  stroke: "#be185d",
})
;"##### ; "215")]
#[test_case(r#####"tw`stroke-pink-800`"#####, r#####"({
  stroke: "#9d174d",
})
;"##### ; "216")]
#[test_case(r#####"tw`stroke-pink-900`"#####, r#####"({
  stroke: "#831843",
})
;"##### ; "217")]
#[test_case(r#####"tw`stroke-rose-50`"#####, r#####"({
  stroke: "#fff1f2",
})
;"##### ; "218")]
#[test_case(r#####"tw`stroke-rose-100`"#####, r#####"({
  stroke: "#ffe4e6",
})
;"##### ; "219")]
#[test_case(r#####"tw`stroke-rose-200`"#####, r#####"({
  stroke: "#fecdd3",
})
;"##### ; "220")]
#[test_case(r#####"tw`stroke-rose-300`"#####, r#####"({
  stroke: "#fda4af",
})
;"##### ; "221")]
#[test_case(r#####"tw`stroke-rose-400`"#####, r#####"({
  stroke: "#fb7185",
})
;"##### ; "222")]
#[test_case(r#####"tw`stroke-rose-500`"#####, r#####"({
  stroke: "#f43f5e",
})
;"##### ; "223")]
#[test_case(r#####"tw`stroke-rose-600`"#####, r#####"({
  stroke: "#e11d48",
})
;"##### ; "224")]
#[test_case(r#####"tw`stroke-rose-700`"#####, r#####"({
  stroke: "#be123c",
})
;"##### ; "225")]
#[test_case(r#####"tw`stroke-rose-800`"#####, r#####"({
  stroke: "#9f1239",
})
;"##### ; "226")]
#[test_case(r#####"tw`stroke-rose-900`"#####, r#####"({
  stroke: "#881337",
})
;"##### ; "227")]
#[test_case(r#####"tw`stroke-[#243c5a]`"#####, r#####"({
  stroke: "#243c5a",
})
;"##### ; "228")]
#[test_case(r#####"tw`stroke-red-500`"#####, r#####"({
  stroke: "#ef4444",
})
;"##### ; "229")]
#[test_case(r#####"tw`stroke-red-500/25`"#####, r#####"({
  stroke: "rgb(239 68 68 / 0.25)",
})
;"##### ; "230")]
#[test_case(r#####"tw`stroke-red-500/fromConfig`"#####, r#####"({
  stroke: "#000",
})
;"##### ; "231")]
#[test_case(r#####"tw`stroke-red-500/fromConfig/25`"#####, r#####"({
  stroke: "rgb(0 0 0 / 0.25)",
})
;"##### ; "232")]
#[test_case(r#####"tw`stroke-red-500/fromConfig/[.555]`"#####, r#####"({
  stroke: "rgb(0 0 0 / .555)",
})
;"##### ; "233")]
#[test_case(r#####"tw`stroke-red-500/fromConfig/[var(--myvar)]`"#####, r#####"({
  stroke: "rgb(0 0 0 / var(--myvar))",
})
;"##### ; "234")]
#[test_case(r#####"tw`stroke-red-500/[.555]`"#####, r#####"({
  stroke: "rgb(239 68 68 / .555)",
})
;"##### ; "235")]
#[test_case(r#####"tw`stroke-red-500/[var(--myvar)]`"#####, r#####"({
  stroke: "rgb(239 68 68 / var(--myvar))",
})
;"##### ; "236")]
#[test_case(r#####"tw`stroke-[theme('colors.red.500')]`"#####, r#####"({
  stroke: "#ef4444",
})
;"##### ; "237")]
#[test_case(r#####"tw`stroke-[theme('colors.red.500')]/20`"#####, r#####"({
  stroke: "rgb(239 68 68 / 0.2)",
})
;"##### ; "238")]
#[test_case(r#####"tw`stroke-electric`"#####, r#####"({
  stroke: "rgb(219, 0, 255)",
})
;"##### ; "239")]
#[test_case(r#####"tw`stroke-electric/25`"#####, r#####"({
  stroke: "rgba(219, 0, 255, 0.25)",
})
;"##### ; "240")]
#[test_case(r#####"tw`stroke-electric/[.555]`"#####, r#####"({
  stroke: "rgba(219, 0, 255, .555)",
})
;"##### ; "241")]
#[test_case(r#####"tw`stroke-electric/[var(--myvar)]`"#####, r#####"({
  stroke: "rgba(219, 0, 255, var(--myvar))",
})
;"##### ; "242")]
#[test_case(r#####"tw`stroke-[theme('colors.electric')]`"#####, r#####"({
  stroke: "rgb(219, 0, 255)",
})
;"##### ; "243")]
#[test_case(r#####"tw`stroke-[theme('colors.electric')]/20`"#####, r#####"({
  stroke: "rgb(219 0 255 / 0.2)",
})
;"##### ; "244")]
#[test_case(r#####"tw`stroke-[color:green]`"#####, r#####"({
  stroke: "green",
})
;"##### ; "245")]
#[test_case(r#####"tw`stroke-[color:rgba(255, 255, 255, .45)]`"#####, r#####"({
  stroke: "rgba(255, 255, 255, .45)",
})
;"##### ; "246")]
#[test_case(r#####"tw`stroke-[length:10px]`"#####, r#####"({
  strokeWidth: "10px",
})"##### ; "247")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
