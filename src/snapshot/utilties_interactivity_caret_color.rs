use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`caretColor`"#####, r#####"({
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
  "red-500/fromConfig": "#000",
  electric: void 0,
})
;"##### ; "0")]
#[test_case(r#####"tw`caret-inherit`"#####, r#####"({
  caretColor: "inherit",
})
;"##### ; "1")]
#[test_case(r#####"tw`caret-current`"#####, r#####"({
  caretColor: "currentColor",
})
;"##### ; "2")]
#[test_case(r#####"tw`caret-transparent`"#####, r#####"({
  caretColor: "transparent",
})
;"##### ; "3")]
#[test_case(r#####"tw`caret-black`"#####, r#####"({
  caretColor: "#000",
})
;"##### ; "4")]
#[test_case(r#####"tw`caret-white`"#####, r#####"({
  caretColor: "#fff",
})
;"##### ; "5")]
#[test_case(r#####"tw`caret-slate-50`"#####, r#####"({
  caretColor: "#f8fafc",
})
;"##### ; "6")]
#[test_case(r#####"tw`caret-slate-100`"#####, r#####"({
  caretColor: "#f1f5f9",
})
;"##### ; "7")]
#[test_case(r#####"tw`caret-slate-200`"#####, r#####"({
  caretColor: "#e2e8f0",
})
;"##### ; "8")]
#[test_case(r#####"tw`caret-slate-300`"#####, r#####"({
  caretColor: "#cbd5e1",
})
;"##### ; "9")]
#[test_case(r#####"tw`caret-slate-400`"#####, r#####"({
  caretColor: "#94a3b8",
})
;"##### ; "10")]
#[test_case(r#####"tw`caret-slate-500`"#####, r#####"({
  caretColor: "#64748b",
})
;"##### ; "11")]
#[test_case(r#####"tw`caret-slate-600`"#####, r#####"({
  caretColor: "#475569",
})
;"##### ; "12")]
#[test_case(r#####"tw`caret-slate-700`"#####, r#####"({
  caretColor: "#334155",
})
;"##### ; "13")]
#[test_case(r#####"tw`caret-slate-800`"#####, r#####"({
  caretColor: "#1e293b",
})
;"##### ; "14")]
#[test_case(r#####"tw`caret-slate-900`"#####, r#####"({
  caretColor: "#0f172a",
})
;"##### ; "15")]
#[test_case(r#####"tw`caret-gray-50`"#####, r#####"({
  caretColor: "#f9fafb",
})
;"##### ; "16")]
#[test_case(r#####"tw`caret-gray-100`"#####, r#####"({
  caretColor: "#f3f4f6",
})
;"##### ; "17")]
#[test_case(r#####"tw`caret-gray-200`"#####, r#####"({
  caretColor: "#e5e7eb",
})
;"##### ; "18")]
#[test_case(r#####"tw`caret-gray-300`"#####, r#####"({
  caretColor: "#d1d5db",
})
;"##### ; "19")]
#[test_case(r#####"tw`caret-gray-400`"#####, r#####"({
  caretColor: "#9ca3af",
})
;"##### ; "20")]
#[test_case(r#####"tw`caret-gray-500`"#####, r#####"({
  caretColor: "#6b7280",
})
;"##### ; "21")]
#[test_case(r#####"tw`caret-gray-600`"#####, r#####"({
  caretColor: "#4b5563",
})
;"##### ; "22")]
#[test_case(r#####"tw`caret-gray-700`"#####, r#####"({
  caretColor: "#374151",
})
;"##### ; "23")]
#[test_case(r#####"tw`caret-gray-800`"#####, r#####"({
  caretColor: "#1f2937",
})
;"##### ; "24")]
#[test_case(r#####"tw`caret-gray-900`"#####, r#####"({
  caretColor: "#111827",
})
;"##### ; "25")]
#[test_case(r#####"tw`caret-zinc-50`"#####, r#####"({
  caretColor: "#fafafa",
})
;"##### ; "26")]
#[test_case(r#####"tw`caret-zinc-100`"#####, r#####"({
  caretColor: "#f4f4f5",
})
;"##### ; "27")]
#[test_case(r#####"tw`caret-zinc-200`"#####, r#####"({
  caretColor: "#e4e4e7",
})
;"##### ; "28")]
#[test_case(r#####"tw`caret-zinc-300`"#####, r#####"({
  caretColor: "#d4d4d8",
})
;"##### ; "29")]
#[test_case(r#####"tw`caret-zinc-400`"#####, r#####"({
  caretColor: "#a1a1aa",
})
;"##### ; "30")]
#[test_case(r#####"tw`caret-zinc-500`"#####, r#####"({
  caretColor: "#71717a",
})
;"##### ; "31")]
#[test_case(r#####"tw`caret-zinc-600`"#####, r#####"({
  caretColor: "#52525b",
})
;"##### ; "32")]
#[test_case(r#####"tw`caret-zinc-700`"#####, r#####"({
  caretColor: "#3f3f46",
})
;"##### ; "33")]
#[test_case(r#####"tw`caret-zinc-800`"#####, r#####"({
  caretColor: "#27272a",
})
;"##### ; "34")]
#[test_case(r#####"tw`caret-zinc-900`"#####, r#####"({
  caretColor: "#18181b",
})
;"##### ; "35")]
#[test_case(r#####"tw`caret-neutral-50`"#####, r#####"({
  caretColor: "#fafafa",
})
;"##### ; "36")]
#[test_case(r#####"tw`caret-neutral-100`"#####, r#####"({
  caretColor: "#f5f5f5",
})
;"##### ; "37")]
#[test_case(r#####"tw`caret-neutral-200`"#####, r#####"({
  caretColor: "#e5e5e5",
})
;"##### ; "38")]
#[test_case(r#####"tw`caret-neutral-300`"#####, r#####"({
  caretColor: "#d4d4d4",
})
;"##### ; "39")]
#[test_case(r#####"tw`caret-neutral-400`"#####, r#####"({
  caretColor: "#a3a3a3",
})
;"##### ; "40")]
#[test_case(r#####"tw`caret-neutral-500`"#####, r#####"({
  caretColor: "#737373",
})
;"##### ; "41")]
#[test_case(r#####"tw`caret-neutral-600`"#####, r#####"({
  caretColor: "#525252",
})
;"##### ; "42")]
#[test_case(r#####"tw`caret-neutral-700`"#####, r#####"({
  caretColor: "#404040",
})
;"##### ; "43")]
#[test_case(r#####"tw`caret-neutral-800`"#####, r#####"({
  caretColor: "#262626",
})
;"##### ; "44")]
#[test_case(r#####"tw`caret-neutral-900`"#####, r#####"({
  caretColor: "#171717",
})
;"##### ; "45")]
#[test_case(r#####"tw`caret-stone-50`"#####, r#####"({
  caretColor: "#fafaf9",
})
;"##### ; "46")]
#[test_case(r#####"tw`caret-stone-100`"#####, r#####"({
  caretColor: "#f5f5f4",
})
;"##### ; "47")]
#[test_case(r#####"tw`caret-stone-200`"#####, r#####"({
  caretColor: "#e7e5e4",
})
;"##### ; "48")]
#[test_case(r#####"tw`caret-stone-300`"#####, r#####"({
  caretColor: "#d6d3d1",
})
;"##### ; "49")]
#[test_case(r#####"tw`caret-stone-400`"#####, r#####"({
  caretColor: "#a8a29e",
})
;"##### ; "50")]
#[test_case(r#####"tw`caret-stone-500`"#####, r#####"({
  caretColor: "#78716c",
})
;"##### ; "51")]
#[test_case(r#####"tw`caret-stone-600`"#####, r#####"({
  caretColor: "#57534e",
})
;"##### ; "52")]
#[test_case(r#####"tw`caret-stone-700`"#####, r#####"({
  caretColor: "#44403c",
})
;"##### ; "53")]
#[test_case(r#####"tw`caret-stone-800`"#####, r#####"({
  caretColor: "#292524",
})
;"##### ; "54")]
#[test_case(r#####"tw`caret-stone-900`"#####, r#####"({
  caretColor: "#1c1917",
})
;"##### ; "55")]
#[test_case(r#####"tw`caret-red-50`"#####, r#####"({
  caretColor: "#fef2f2",
})
;"##### ; "56")]
#[test_case(r#####"tw`caret-red-100`"#####, r#####"({
  caretColor: "#fee2e2",
})
;"##### ; "57")]
#[test_case(r#####"tw`caret-red-200`"#####, r#####"({
  caretColor: "#fecaca",
})
;"##### ; "58")]
#[test_case(r#####"tw`caret-red-300`"#####, r#####"({
  caretColor: "#fca5a5",
})
;"##### ; "59")]
#[test_case(r#####"tw`caret-red-400`"#####, r#####"({
  caretColor: "#f87171",
})
;"##### ; "60")]
#[test_case(r#####"tw`caret-red-500`"#####, r#####"({
  caretColor: "#ef4444",
})
;"##### ; "61")]
#[test_case(r#####"tw`caret-red-600`"#####, r#####"({
  caretColor: "#dc2626",
})
;"##### ; "62")]
#[test_case(r#####"tw`caret-red-700`"#####, r#####"({
  caretColor: "#b91c1c",
})
;"##### ; "63")]
#[test_case(r#####"tw`caret-red-800`"#####, r#####"({
  caretColor: "#991b1b",
})
;"##### ; "64")]
#[test_case(r#####"tw`caret-red-900`"#####, r#####"({
  caretColor: "#7f1d1d",
})
;"##### ; "65")]
#[test_case(r#####"tw`caret-orange-50`"#####, r#####"({
  caretColor: "#fff7ed",
})
;"##### ; "66")]
#[test_case(r#####"tw`caret-orange-100`"#####, r#####"({
  caretColor: "#ffedd5",
})
;"##### ; "67")]
#[test_case(r#####"tw`caret-orange-200`"#####, r#####"({
  caretColor: "#fed7aa",
})
;"##### ; "68")]
#[test_case(r#####"tw`caret-orange-300`"#####, r#####"({
  caretColor: "#fdba74",
})
;"##### ; "69")]
#[test_case(r#####"tw`caret-orange-400`"#####, r#####"({
  caretColor: "#fb923c",
})
;"##### ; "70")]
#[test_case(r#####"tw`caret-orange-500`"#####, r#####"({
  caretColor: "#f97316",
})
;"##### ; "71")]
#[test_case(r#####"tw`caret-orange-600`"#####, r#####"({
  caretColor: "#ea580c",
})
;"##### ; "72")]
#[test_case(r#####"tw`caret-orange-700`"#####, r#####"({
  caretColor: "#c2410c",
})
;"##### ; "73")]
#[test_case(r#####"tw`caret-orange-800`"#####, r#####"({
  caretColor: "#9a3412",
})
;"##### ; "74")]
#[test_case(r#####"tw`caret-orange-900`"#####, r#####"({
  caretColor: "#7c2d12",
})
;"##### ; "75")]
#[test_case(r#####"tw`caret-amber-50`"#####, r#####"({
  caretColor: "#fffbeb",
})
;"##### ; "76")]
#[test_case(r#####"tw`caret-amber-100`"#####, r#####"({
  caretColor: "#fef3c7",
})
;"##### ; "77")]
#[test_case(r#####"tw`caret-amber-200`"#####, r#####"({
  caretColor: "#fde68a",
})
;"##### ; "78")]
#[test_case(r#####"tw`caret-amber-300`"#####, r#####"({
  caretColor: "#fcd34d",
})
;"##### ; "79")]
#[test_case(r#####"tw`caret-amber-400`"#####, r#####"({
  caretColor: "#fbbf24",
})
;"##### ; "80")]
#[test_case(r#####"tw`caret-amber-500`"#####, r#####"({
  caretColor: "#f59e0b",
})
;"##### ; "81")]
#[test_case(r#####"tw`caret-amber-600`"#####, r#####"({
  caretColor: "#d97706",
})
;"##### ; "82")]
#[test_case(r#####"tw`caret-amber-700`"#####, r#####"({
  caretColor: "#b45309",
})
;"##### ; "83")]
#[test_case(r#####"tw`caret-amber-800`"#####, r#####"({
  caretColor: "#92400e",
})
;"##### ; "84")]
#[test_case(r#####"tw`caret-amber-900`"#####, r#####"({
  caretColor: "#78350f",
})
;"##### ; "85")]
#[test_case(r#####"tw`caret-yellow-50`"#####, r#####"({
  caretColor: "#fefce8",
})
;"##### ; "86")]
#[test_case(r#####"tw`caret-yellow-100`"#####, r#####"({
  caretColor: "#fef9c3",
})
;"##### ; "87")]
#[test_case(r#####"tw`caret-yellow-200`"#####, r#####"({
  caretColor: "#fef08a",
})
;"##### ; "88")]
#[test_case(r#####"tw`caret-yellow-300`"#####, r#####"({
  caretColor: "#fde047",
})
;"##### ; "89")]
#[test_case(r#####"tw`caret-yellow-400`"#####, r#####"({
  caretColor: "#facc15",
})
;"##### ; "90")]
#[test_case(r#####"tw`caret-yellow-500`"#####, r#####"({
  caretColor: "#eab308",
})
;"##### ; "91")]
#[test_case(r#####"tw`caret-yellow-600`"#####, r#####"({
  caretColor: "#ca8a04",
})
;"##### ; "92")]
#[test_case(r#####"tw`caret-yellow-700`"#####, r#####"({
  caretColor: "#a16207",
})
;"##### ; "93")]
#[test_case(r#####"tw`caret-yellow-800`"#####, r#####"({
  caretColor: "#854d0e",
})
;"##### ; "94")]
#[test_case(r#####"tw`caret-yellow-900`"#####, r#####"({
  caretColor: "#713f12",
})
;"##### ; "95")]
#[test_case(r#####"tw`caret-lime-50`"#####, r#####"({
  caretColor: "#f7fee7",
})
;"##### ; "96")]
#[test_case(r#####"tw`caret-lime-100`"#####, r#####"({
  caretColor: "#ecfccb",
})
;"##### ; "97")]
#[test_case(r#####"tw`caret-lime-200`"#####, r#####"({
  caretColor: "#d9f99d",
})
;"##### ; "98")]
#[test_case(r#####"tw`caret-lime-300`"#####, r#####"({
  caretColor: "#bef264",
})
;"##### ; "99")]
#[test_case(r#####"tw`caret-lime-400`"#####, r#####"({
  caretColor: "#a3e635",
})
;"##### ; "100")]
#[test_case(r#####"tw`caret-lime-500`"#####, r#####"({
  caretColor: "#84cc16",
})
;"##### ; "101")]
#[test_case(r#####"tw`caret-lime-600`"#####, r#####"({
  caretColor: "#65a30d",
})
;"##### ; "102")]
#[test_case(r#####"tw`caret-lime-700`"#####, r#####"({
  caretColor: "#4d7c0f",
})
;"##### ; "103")]
#[test_case(r#####"tw`caret-lime-800`"#####, r#####"({
  caretColor: "#3f6212",
})
;"##### ; "104")]
#[test_case(r#####"tw`caret-lime-900`"#####, r#####"({
  caretColor: "#365314",
})
;"##### ; "105")]
#[test_case(r#####"tw`caret-green-50`"#####, r#####"({
  caretColor: "#f0fdf4",
})
;"##### ; "106")]
#[test_case(r#####"tw`caret-green-100`"#####, r#####"({
  caretColor: "#dcfce7",
})
;"##### ; "107")]
#[test_case(r#####"tw`caret-green-200`"#####, r#####"({
  caretColor: "#bbf7d0",
})
;"##### ; "108")]
#[test_case(r#####"tw`caret-green-300`"#####, r#####"({
  caretColor: "#86efac",
})
;"##### ; "109")]
#[test_case(r#####"tw`caret-green-400`"#####, r#####"({
  caretColor: "#4ade80",
})
;"##### ; "110")]
#[test_case(r#####"tw`caret-green-500`"#####, r#####"({
  caretColor: "#22c55e",
})
;"##### ; "111")]
#[test_case(r#####"tw`caret-green-600`"#####, r#####"({
  caretColor: "#16a34a",
})
;"##### ; "112")]
#[test_case(r#####"tw`caret-green-700`"#####, r#####"({
  caretColor: "#15803d",
})
;"##### ; "113")]
#[test_case(r#####"tw`caret-green-800`"#####, r#####"({
  caretColor: "#166534",
})
;"##### ; "114")]
#[test_case(r#####"tw`caret-green-900`"#####, r#####"({
  caretColor: "#14532d",
})
;"##### ; "115")]
#[test_case(r#####"tw`caret-emerald-50`"#####, r#####"({
  caretColor: "#ecfdf5",
})
;"##### ; "116")]
#[test_case(r#####"tw`caret-emerald-100`"#####, r#####"({
  caretColor: "#d1fae5",
})
;"##### ; "117")]
#[test_case(r#####"tw`caret-emerald-200`"#####, r#####"({
  caretColor: "#a7f3d0",
})
;"##### ; "118")]
#[test_case(r#####"tw`caret-emerald-300`"#####, r#####"({
  caretColor: "#6ee7b7",
})
;"##### ; "119")]
#[test_case(r#####"tw`caret-emerald-400`"#####, r#####"({
  caretColor: "#34d399",
})
;"##### ; "120")]
#[test_case(r#####"tw`caret-emerald-500`"#####, r#####"({
  caretColor: "#10b981",
})
;"##### ; "121")]
#[test_case(r#####"tw`caret-emerald-600`"#####, r#####"({
  caretColor: "#059669",
})
;"##### ; "122")]
#[test_case(r#####"tw`caret-emerald-700`"#####, r#####"({
  caretColor: "#047857",
})
;"##### ; "123")]
#[test_case(r#####"tw`caret-emerald-800`"#####, r#####"({
  caretColor: "#065f46",
})
;"##### ; "124")]
#[test_case(r#####"tw`caret-emerald-900`"#####, r#####"({
  caretColor: "#064e3b",
})
;"##### ; "125")]
#[test_case(r#####"tw`caret-teal-50`"#####, r#####"({
  caretColor: "#f0fdfa",
})
;"##### ; "126")]
#[test_case(r#####"tw`caret-teal-100`"#####, r#####"({
  caretColor: "#ccfbf1",
})
;"##### ; "127")]
#[test_case(r#####"tw`caret-teal-200`"#####, r#####"({
  caretColor: "#99f6e4",
})
;"##### ; "128")]
#[test_case(r#####"tw`caret-teal-300`"#####, r#####"({
  caretColor: "#5eead4",
})
;"##### ; "129")]
#[test_case(r#####"tw`caret-teal-400`"#####, r#####"({
  caretColor: "#2dd4bf",
})
;"##### ; "130")]
#[test_case(r#####"tw`caret-teal-500`"#####, r#####"({
  caretColor: "#14b8a6",
})
;"##### ; "131")]
#[test_case(r#####"tw`caret-teal-600`"#####, r#####"({
  caretColor: "#0d9488",
})
;"##### ; "132")]
#[test_case(r#####"tw`caret-teal-700`"#####, r#####"({
  caretColor: "#0f766e",
})
;"##### ; "133")]
#[test_case(r#####"tw`caret-teal-800`"#####, r#####"({
  caretColor: "#115e59",
})
;"##### ; "134")]
#[test_case(r#####"tw`caret-teal-900`"#####, r#####"({
  caretColor: "#134e4a",
})
;"##### ; "135")]
#[test_case(r#####"tw`caret-cyan-50`"#####, r#####"({
  caretColor: "#ecfeff",
})
;"##### ; "136")]
#[test_case(r#####"tw`caret-cyan-100`"#####, r#####"({
  caretColor: "#cffafe",
})
;"##### ; "137")]
#[test_case(r#####"tw`caret-cyan-200`"#####, r#####"({
  caretColor: "#a5f3fc",
})
;"##### ; "138")]
#[test_case(r#####"tw`caret-cyan-300`"#####, r#####"({
  caretColor: "#67e8f9",
})
;"##### ; "139")]
#[test_case(r#####"tw`caret-cyan-400`"#####, r#####"({
  caretColor: "#22d3ee",
})
;"##### ; "140")]
#[test_case(r#####"tw`caret-cyan-500`"#####, r#####"({
  caretColor: "#06b6d4",
})
;"##### ; "141")]
#[test_case(r#####"tw`caret-cyan-600`"#####, r#####"({
  caretColor: "#0891b2",
})
;"##### ; "142")]
#[test_case(r#####"tw`caret-cyan-700`"#####, r#####"({
  caretColor: "#0e7490",
})
;"##### ; "143")]
#[test_case(r#####"tw`caret-cyan-800`"#####, r#####"({
  caretColor: "#155e75",
})
;"##### ; "144")]
#[test_case(r#####"tw`caret-cyan-900`"#####, r#####"({
  caretColor: "#164e63",
})
;"##### ; "145")]
#[test_case(r#####"tw`caret-sky-50`"#####, r#####"({
  caretColor: "#f0f9ff",
})
;"##### ; "146")]
#[test_case(r#####"tw`caret-sky-100`"#####, r#####"({
  caretColor: "#e0f2fe",
})
;"##### ; "147")]
#[test_case(r#####"tw`caret-sky-200`"#####, r#####"({
  caretColor: "#bae6fd",
})
;"##### ; "148")]
#[test_case(r#####"tw`caret-sky-300`"#####, r#####"({
  caretColor: "#7dd3fc",
})
;"##### ; "149")]
#[test_case(r#####"tw`caret-sky-400`"#####, r#####"({
  caretColor: "#38bdf8",
})
;"##### ; "150")]
#[test_case(r#####"tw`caret-sky-500`"#####, r#####"({
  caretColor: "#0ea5e9",
})
;"##### ; "151")]
#[test_case(r#####"tw`caret-sky-600`"#####, r#####"({
  caretColor: "#0284c7",
})
;"##### ; "152")]
#[test_case(r#####"tw`caret-sky-700`"#####, r#####"({
  caretColor: "#0369a1",
})
;"##### ; "153")]
#[test_case(r#####"tw`caret-sky-800`"#####, r#####"({
  caretColor: "#075985",
})
;"##### ; "154")]
#[test_case(r#####"tw`caret-sky-900`"#####, r#####"({
  caretColor: "#0c4a6e",
})
;"##### ; "155")]
#[test_case(r#####"tw`caret-blue-50`"#####, r#####"({
  caretColor: "#eff6ff",
})
;"##### ; "156")]
#[test_case(r#####"tw`caret-blue-100`"#####, r#####"({
  caretColor: "#dbeafe",
})
;"##### ; "157")]
#[test_case(r#####"tw`caret-blue-200`"#####, r#####"({
  caretColor: "#bfdbfe",
})
;"##### ; "158")]
#[test_case(r#####"tw`caret-blue-300`"#####, r#####"({
  caretColor: "#93c5fd",
})
;"##### ; "159")]
#[test_case(r#####"tw`caret-blue-400`"#####, r#####"({
  caretColor: "#60a5fa",
})
;"##### ; "160")]
#[test_case(r#####"tw`caret-blue-500`"#####, r#####"({
  caretColor: "#3b82f6",
})
;"##### ; "161")]
#[test_case(r#####"tw`caret-blue-600`"#####, r#####"({
  caretColor: "#2563eb",
})
;"##### ; "162")]
#[test_case(r#####"tw`caret-blue-700`"#####, r#####"({
  caretColor: "#1d4ed8",
})
;"##### ; "163")]
#[test_case(r#####"tw`caret-blue-800`"#####, r#####"({
  caretColor: "#1e40af",
})
;"##### ; "164")]
#[test_case(r#####"tw`caret-blue-900`"#####, r#####"({
  caretColor: "#1e3a8a",
})
;"##### ; "165")]
#[test_case(r#####"tw`caret-indigo-50`"#####, r#####"({
  caretColor: "#eef2ff",
})
;"##### ; "166")]
#[test_case(r#####"tw`caret-indigo-100`"#####, r#####"({
  caretColor: "#e0e7ff",
})
;"##### ; "167")]
#[test_case(r#####"tw`caret-indigo-200`"#####, r#####"({
  caretColor: "#c7d2fe",
})
;"##### ; "168")]
#[test_case(r#####"tw`caret-indigo-300`"#####, r#####"({
  caretColor: "#a5b4fc",
})
;"##### ; "169")]
#[test_case(r#####"tw`caret-indigo-400`"#####, r#####"({
  caretColor: "#818cf8",
})
;"##### ; "170")]
#[test_case(r#####"tw`caret-indigo-500`"#####, r#####"({
  caretColor: "#6366f1",
})
;"##### ; "171")]
#[test_case(r#####"tw`caret-indigo-600`"#####, r#####"({
  caretColor: "#4f46e5",
})
;"##### ; "172")]
#[test_case(r#####"tw`caret-indigo-700`"#####, r#####"({
  caretColor: "#4338ca",
})
;"##### ; "173")]
#[test_case(r#####"tw`caret-indigo-800`"#####, r#####"({
  caretColor: "#3730a3",
})
;"##### ; "174")]
#[test_case(r#####"tw`caret-indigo-900`"#####, r#####"({
  caretColor: "#312e81",
})
;"##### ; "175")]
#[test_case(r#####"tw`caret-violet-50`"#####, r#####"({
  caretColor: "#f5f3ff",
})
;"##### ; "176")]
#[test_case(r#####"tw`caret-violet-100`"#####, r#####"({
  caretColor: "#ede9fe",
})
;"##### ; "177")]
#[test_case(r#####"tw`caret-violet-200`"#####, r#####"({
  caretColor: "#ddd6fe",
})
;"##### ; "178")]
#[test_case(r#####"tw`caret-violet-300`"#####, r#####"({
  caretColor: "#c4b5fd",
})
;"##### ; "179")]
#[test_case(r#####"tw`caret-violet-400`"#####, r#####"({
  caretColor: "#a78bfa",
})
;"##### ; "180")]
#[test_case(r#####"tw`caret-violet-500`"#####, r#####"({
  caretColor: "#8b5cf6",
})
;"##### ; "181")]
#[test_case(r#####"tw`caret-violet-600`"#####, r#####"({
  caretColor: "#7c3aed",
})
;"##### ; "182")]
#[test_case(r#####"tw`caret-violet-700`"#####, r#####"({
  caretColor: "#6d28d9",
})
;"##### ; "183")]
#[test_case(r#####"tw`caret-violet-800`"#####, r#####"({
  caretColor: "#5b21b6",
})
;"##### ; "184")]
#[test_case(r#####"tw`caret-violet-900`"#####, r#####"({
  caretColor: "#4c1d95",
})
;"##### ; "185")]
#[test_case(r#####"tw`caret-purple-50`"#####, r#####"({
  caretColor: "#faf5ff",
})
;"##### ; "186")]
#[test_case(r#####"tw`caret-purple-100`"#####, r#####"({
  caretColor: "#f3e8ff",
})
;"##### ; "187")]
#[test_case(r#####"tw`caret-purple-200`"#####, r#####"({
  caretColor: "#e9d5ff",
})
;"##### ; "188")]
#[test_case(r#####"tw`caret-purple-300`"#####, r#####"({
  caretColor: "#d8b4fe",
})
;"##### ; "189")]
#[test_case(r#####"tw`caret-purple-400`"#####, r#####"({
  caretColor: "#c084fc",
})
;"##### ; "190")]
#[test_case(r#####"tw`caret-purple-500`"#####, r#####"({
  caretColor: "#a855f7",
})
;"##### ; "191")]
#[test_case(r#####"tw`caret-purple-600`"#####, r#####"({
  caretColor: "#9333ea",
})
;"##### ; "192")]
#[test_case(r#####"tw`caret-purple-700`"#####, r#####"({
  caretColor: "#7e22ce",
})
;"##### ; "193")]
#[test_case(r#####"tw`caret-purple-800`"#####, r#####"({
  caretColor: "#6b21a8",
})
;"##### ; "194")]
#[test_case(r#####"tw`caret-purple-900`"#####, r#####"({
  caretColor: "#581c87",
})
;"##### ; "195")]
#[test_case(r#####"tw`caret-fuchsia-50`"#####, r#####"({
  caretColor: "#fdf4ff",
})
;"##### ; "196")]
#[test_case(r#####"tw`caret-fuchsia-100`"#####, r#####"({
  caretColor: "#fae8ff",
})
;"##### ; "197")]
#[test_case(r#####"tw`caret-fuchsia-200`"#####, r#####"({
  caretColor: "#f5d0fe",
})
;"##### ; "198")]
#[test_case(r#####"tw`caret-fuchsia-300`"#####, r#####"({
  caretColor: "#f0abfc",
})
;"##### ; "199")]
#[test_case(r#####"tw`caret-fuchsia-400`"#####, r#####"({
  caretColor: "#e879f9",
})
;"##### ; "200")]
#[test_case(r#####"tw`caret-fuchsia-500`"#####, r#####"({
  caretColor: "#d946ef",
})
;"##### ; "201")]
#[test_case(r#####"tw`caret-fuchsia-600`"#####, r#####"({
  caretColor: "#c026d3",
})
;"##### ; "202")]
#[test_case(r#####"tw`caret-fuchsia-700`"#####, r#####"({
  caretColor: "#a21caf",
})
;"##### ; "203")]
#[test_case(r#####"tw`caret-fuchsia-800`"#####, r#####"({
  caretColor: "#86198f",
})
;"##### ; "204")]
#[test_case(r#####"tw`caret-fuchsia-900`"#####, r#####"({
  caretColor: "#701a75",
})
;"##### ; "205")]
#[test_case(r#####"tw`caret-pink-50`"#####, r#####"({
  caretColor: "#fdf2f8",
})
;"##### ; "206")]
#[test_case(r#####"tw`caret-pink-100`"#####, r#####"({
  caretColor: "#fce7f3",
})
;"##### ; "207")]
#[test_case(r#####"tw`caret-pink-200`"#####, r#####"({
  caretColor: "#fbcfe8",
})
;"##### ; "208")]
#[test_case(r#####"tw`caret-pink-300`"#####, r#####"({
  caretColor: "#f9a8d4",
})
;"##### ; "209")]
#[test_case(r#####"tw`caret-pink-400`"#####, r#####"({
  caretColor: "#f472b6",
})
;"##### ; "210")]
#[test_case(r#####"tw`caret-pink-500`"#####, r#####"({
  caretColor: "#ec4899",
})
;"##### ; "211")]
#[test_case(r#####"tw`caret-pink-600`"#####, r#####"({
  caretColor: "#db2777",
})
;"##### ; "212")]
#[test_case(r#####"tw`caret-pink-700`"#####, r#####"({
  caretColor: "#be185d",
})
;"##### ; "213")]
#[test_case(r#####"tw`caret-pink-800`"#####, r#####"({
  caretColor: "#9d174d",
})
;"##### ; "214")]
#[test_case(r#####"tw`caret-pink-900`"#####, r#####"({
  caretColor: "#831843",
})
;"##### ; "215")]
#[test_case(r#####"tw`caret-rose-50`"#####, r#####"({
  caretColor: "#fff1f2",
})
;"##### ; "216")]
#[test_case(r#####"tw`caret-rose-100`"#####, r#####"({
  caretColor: "#ffe4e6",
})
;"##### ; "217")]
#[test_case(r#####"tw`caret-rose-200`"#####, r#####"({
  caretColor: "#fecdd3",
})
;"##### ; "218")]
#[test_case(r#####"tw`caret-rose-300`"#####, r#####"({
  caretColor: "#fda4af",
})
;"##### ; "219")]
#[test_case(r#####"tw`caret-rose-400`"#####, r#####"({
  caretColor: "#fb7185",
})
;"##### ; "220")]
#[test_case(r#####"tw`caret-rose-500`"#####, r#####"({
  caretColor: "#f43f5e",
})
;"##### ; "221")]
#[test_case(r#####"tw`caret-rose-600`"#####, r#####"({
  caretColor: "#e11d48",
})
;"##### ; "222")]
#[test_case(r#####"tw`caret-rose-700`"#####, r#####"({
  caretColor: "#be123c",
})
;"##### ; "223")]
#[test_case(r#####"tw`caret-rose-800`"#####, r#####"({
  caretColor: "#9f1239",
})
;"##### ; "224")]
#[test_case(r#####"tw`caret-rose-900`"#####, r#####"({
  caretColor: "#881337",
})
;"##### ; "225")]
#[test_case(r#####"tw`caret-[#50d71e]`"#####, r#####"({
  caretColor: "#50d71e",
})
;"##### ; "226")]
#[test_case(r#####"tw`caret-red-500/25`"#####, r#####"({
  caretColor: "rgb(239 68 68 / 0.25)",
})
;"##### ; "227")]
#[test_case(r#####"tw`caret-red-500/fromConfig`"#####, r#####"({
  caretColor: "#000",
})
;"##### ; "228")]
#[test_case(r#####"tw`caret-red-500/fromConfig/25`"#####, r#####"({
  caretColor: "rgb(0 0 0 / 0.25)",
})
;"##### ; "229")]
#[test_case(r#####"tw`caret-red-500/fromConfig/[.555]`"#####, r#####"({
  caretColor: "rgb(0 0 0 / .555)",
})
;"##### ; "230")]
#[test_case(r#####"tw`caret-red-500/fromConfig/[var(--myvar)]`"#####, r#####"({
  caretColor: "rgb(0 0 0 / var(--myvar))",
})
;"##### ; "231")]
#[test_case(r#####"tw`caret-red-500/[.555]`"#####, r#####"({
  caretColor: "rgb(239 68 68 / .555)",
})
;"##### ; "232")]
#[test_case(r#####"tw`caret-red-500/[var(--myvar)]`"#####, r#####"({
  caretColor: "rgb(239 68 68 / var(--myvar))",
})
;"##### ; "233")]
#[test_case(r#####"tw`caret-[theme('colors.red.500')]`"#####, r#####"({
  caretColor: "#ef4444",
})
;"##### ; "234")]
#[test_case(r#####"tw`caret-[theme('colors.red.500')]/20`"#####, r#####"({
  caretColor: "rgb(239 68 68 / 0.2)",
})
;"##### ; "235")]
#[test_case(r#####"tw`caret-electric`"#####, r#####"({
  caretColor: "rgb(219, 0, 255)",
})
;"##### ; "236")]
#[test_case(r#####"tw`caret-electric/25`"#####, r#####"({
  caretColor: "rgba(219, 0, 255, 0.25)",
})
;"##### ; "237")]
#[test_case(r#####"tw`caret-electric/[.555]`"#####, r#####"({
  caretColor: "rgba(219, 0, 255, .555)",
})
;"##### ; "238")]
#[test_case(r#####"tw`caret-electric/[var(--myvar)]`"#####, r#####"({
  caretColor: "rgba(219, 0, 255, var(--myvar))",
})
;"##### ; "239")]
#[test_case(r#####"tw`caret-[theme('colors.electric')]`"#####, r#####"({
  caretColor: "rgb(219, 0, 255)",
})
;"##### ; "240")]
#[test_case(r#####"tw`caret-[theme('colors.electric')]/20`"#####, r#####"({
  caretColor: "rgb(219 0 255 / 0.2)",
})
;"##### ; "241")]
#[test_case(r#####"tw`caret-[color:green]`"#####, r#####"({
  caretColor: "green",
})
;"##### ; "242")]
#[test_case(r#####"tw`caret-[color:rgba(255, 255, 255, .45)]`"#####, r#####"({
  caretColor: "rgba(255, 255, 255, .45)",
})
;"##### ; "243")]
#[test_case(r#####"tw`caret-[any:blah]`"#####, r#####"({
  caretColor: "blah",
})"##### ; "244")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
