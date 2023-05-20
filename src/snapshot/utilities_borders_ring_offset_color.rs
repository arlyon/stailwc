use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`ringOffsetColor`"#####, r#####"({
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
#[test_case(r#####"tw`ring-offset-inherit`"#####, r#####"({
  '--tw-ring-offset-color': "inherit",
})
;"##### ; "1")]
#[test_case(r#####"tw`ring-offset-current`"#####, r#####"({
  '--tw-ring-offset-color': "currentColor",
})
;"##### ; "2")]
#[test_case(r#####"tw`ring-offset-transparent`"#####, r#####"({
  '--tw-ring-offset-color': "transparent",
})
;"##### ; "3")]
#[test_case(r#####"tw`ring-offset-black`"#####, r#####"({
  '--tw-ring-offset-color': "#000",
})
;"##### ; "4")]
#[test_case(r#####"tw`ring-offset-white`"#####, r#####"({
  '--tw-ring-offset-color': "#fff",
})
;"##### ; "5")]
#[test_case(r#####"tw`ring-offset-slate-50`"#####, r#####"({
  '--tw-ring-offset-color': "#f8fafc",
})
;"##### ; "6")]
#[test_case(r#####"tw`ring-offset-slate-100`"#####, r#####"({
  '--tw-ring-offset-color': "#f1f5f9",
})
;"##### ; "7")]
#[test_case(r#####"tw`ring-offset-slate-200`"#####, r#####"({
  '--tw-ring-offset-color': "#e2e8f0",
})
;"##### ; "8")]
#[test_case(r#####"tw`ring-offset-slate-300`"#####, r#####"({
  '--tw-ring-offset-color': "#cbd5e1",
})
;"##### ; "9")]
#[test_case(r#####"tw`ring-offset-slate-400`"#####, r#####"({
  '--tw-ring-offset-color': "#94a3b8",
})
;"##### ; "10")]
#[test_case(r#####"tw`ring-offset-slate-500`"#####, r#####"({
  '--tw-ring-offset-color': "#64748b",
})
;"##### ; "11")]
#[test_case(r#####"tw`ring-offset-slate-600`"#####, r#####"({
  '--tw-ring-offset-color': "#475569",
})
;"##### ; "12")]
#[test_case(r#####"tw`ring-offset-slate-700`"#####, r#####"({
  '--tw-ring-offset-color': "#334155",
})
;"##### ; "13")]
#[test_case(r#####"tw`ring-offset-slate-800`"#####, r#####"({
  '--tw-ring-offset-color': "#1e293b",
})
;"##### ; "14")]
#[test_case(r#####"tw`ring-offset-slate-900`"#####, r#####"({
  '--tw-ring-offset-color': "#0f172a",
})
;"##### ; "15")]
#[test_case(r#####"tw`ring-offset-gray-50`"#####, r#####"({
  '--tw-ring-offset-color': "#f9fafb",
})
;"##### ; "16")]
#[test_case(r#####"tw`ring-offset-gray-100`"#####, r#####"({
  '--tw-ring-offset-color': "#f3f4f6",
})
;"##### ; "17")]
#[test_case(r#####"tw`ring-offset-gray-200`"#####, r#####"({
  '--tw-ring-offset-color': "#e5e7eb",
})
;"##### ; "18")]
#[test_case(r#####"tw`ring-offset-gray-300`"#####, r#####"({
  '--tw-ring-offset-color': "#d1d5db",
})
;"##### ; "19")]
#[test_case(r#####"tw`ring-offset-gray-400`"#####, r#####"({
  '--tw-ring-offset-color': "#9ca3af",
})
;"##### ; "20")]
#[test_case(r#####"tw`ring-offset-gray-500`"#####, r#####"({
  '--tw-ring-offset-color': "#6b7280",
})
;"##### ; "21")]
#[test_case(r#####"tw`ring-offset-gray-600`"#####, r#####"({
  '--tw-ring-offset-color': "#4b5563",
})
;"##### ; "22")]
#[test_case(r#####"tw`ring-offset-gray-700`"#####, r#####"({
  '--tw-ring-offset-color': "#374151",
})
;"##### ; "23")]
#[test_case(r#####"tw`ring-offset-gray-800`"#####, r#####"({
  '--tw-ring-offset-color': "#1f2937",
})
;"##### ; "24")]
#[test_case(r#####"tw`ring-offset-gray-900`"#####, r#####"({
  '--tw-ring-offset-color': "#111827",
})
;"##### ; "25")]
#[test_case(r#####"tw`ring-offset-zinc-50`"#####, r#####"({
  '--tw-ring-offset-color': "#fafafa",
})
;"##### ; "26")]
#[test_case(r#####"tw`ring-offset-zinc-100`"#####, r#####"({
  '--tw-ring-offset-color': "#f4f4f5",
})
;"##### ; "27")]
#[test_case(r#####"tw`ring-offset-zinc-200`"#####, r#####"({
  '--tw-ring-offset-color': "#e4e4e7",
})
;"##### ; "28")]
#[test_case(r#####"tw`ring-offset-zinc-300`"#####, r#####"({
  '--tw-ring-offset-color': "#d4d4d8",
})
;"##### ; "29")]
#[test_case(r#####"tw`ring-offset-zinc-400`"#####, r#####"({
  '--tw-ring-offset-color': "#a1a1aa",
})
;"##### ; "30")]
#[test_case(r#####"tw`ring-offset-zinc-500`"#####, r#####"({
  '--tw-ring-offset-color': "#71717a",
})
;"##### ; "31")]
#[test_case(r#####"tw`ring-offset-zinc-600`"#####, r#####"({
  '--tw-ring-offset-color': "#52525b",
})
;"##### ; "32")]
#[test_case(r#####"tw`ring-offset-zinc-700`"#####, r#####"({
  '--tw-ring-offset-color': "#3f3f46",
})
;"##### ; "33")]
#[test_case(r#####"tw`ring-offset-zinc-800`"#####, r#####"({
  '--tw-ring-offset-color': "#27272a",
})
;"##### ; "34")]
#[test_case(r#####"tw`ring-offset-zinc-900`"#####, r#####"({
  '--tw-ring-offset-color': "#18181b",
})
;"##### ; "35")]
#[test_case(r#####"tw`ring-offset-neutral-50`"#####, r#####"({
  '--tw-ring-offset-color': "#fafafa",
})
;"##### ; "36")]
#[test_case(r#####"tw`ring-offset-neutral-100`"#####, r#####"({
  '--tw-ring-offset-color': "#f5f5f5",
})
;"##### ; "37")]
#[test_case(r#####"tw`ring-offset-neutral-200`"#####, r#####"({
  '--tw-ring-offset-color': "#e5e5e5",
})
;"##### ; "38")]
#[test_case(r#####"tw`ring-offset-neutral-300`"#####, r#####"({
  '--tw-ring-offset-color': "#d4d4d4",
})
;"##### ; "39")]
#[test_case(r#####"tw`ring-offset-neutral-400`"#####, r#####"({
  '--tw-ring-offset-color': "#a3a3a3",
})
;"##### ; "40")]
#[test_case(r#####"tw`ring-offset-neutral-500`"#####, r#####"({
  '--tw-ring-offset-color': "#737373",
})
;"##### ; "41")]
#[test_case(r#####"tw`ring-offset-neutral-600`"#####, r#####"({
  '--tw-ring-offset-color': "#525252",
})
;"##### ; "42")]
#[test_case(r#####"tw`ring-offset-neutral-700`"#####, r#####"({
  '--tw-ring-offset-color': "#404040",
})
;"##### ; "43")]
#[test_case(r#####"tw`ring-offset-neutral-800`"#####, r#####"({
  '--tw-ring-offset-color': "#262626",
})
;"##### ; "44")]
#[test_case(r#####"tw`ring-offset-neutral-900`"#####, r#####"({
  '--tw-ring-offset-color': "#171717",
})
;"##### ; "45")]
#[test_case(r#####"tw`ring-offset-stone-50`"#####, r#####"({
  '--tw-ring-offset-color': "#fafaf9",
})
;"##### ; "46")]
#[test_case(r#####"tw`ring-offset-stone-100`"#####, r#####"({
  '--tw-ring-offset-color': "#f5f5f4",
})
;"##### ; "47")]
#[test_case(r#####"tw`ring-offset-stone-200`"#####, r#####"({
  '--tw-ring-offset-color': "#e7e5e4",
})
;"##### ; "48")]
#[test_case(r#####"tw`ring-offset-stone-300`"#####, r#####"({
  '--tw-ring-offset-color': "#d6d3d1",
})
;"##### ; "49")]
#[test_case(r#####"tw`ring-offset-stone-400`"#####, r#####"({
  '--tw-ring-offset-color': "#a8a29e",
})
;"##### ; "50")]
#[test_case(r#####"tw`ring-offset-stone-500`"#####, r#####"({
  '--tw-ring-offset-color': "#78716c",
})
;"##### ; "51")]
#[test_case(r#####"tw`ring-offset-stone-600`"#####, r#####"({
  '--tw-ring-offset-color': "#57534e",
})
;"##### ; "52")]
#[test_case(r#####"tw`ring-offset-stone-700`"#####, r#####"({
  '--tw-ring-offset-color': "#44403c",
})
;"##### ; "53")]
#[test_case(r#####"tw`ring-offset-stone-800`"#####, r#####"({
  '--tw-ring-offset-color': "#292524",
})
;"##### ; "54")]
#[test_case(r#####"tw`ring-offset-stone-900`"#####, r#####"({
  '--tw-ring-offset-color': "#1c1917",
})
;"##### ; "55")]
#[test_case(r#####"tw`ring-offset-red-50`"#####, r#####"({
  '--tw-ring-offset-color': "#fef2f2",
})
;"##### ; "56")]
#[test_case(r#####"tw`ring-offset-red-100`"#####, r#####"({
  '--tw-ring-offset-color': "#fee2e2",
})
;"##### ; "57")]
#[test_case(r#####"tw`ring-offset-red-200`"#####, r#####"({
  '--tw-ring-offset-color': "#fecaca",
})
;"##### ; "58")]
#[test_case(r#####"tw`ring-offset-red-300`"#####, r#####"({
  '--tw-ring-offset-color': "#fca5a5",
})
;"##### ; "59")]
#[test_case(r#####"tw`ring-offset-red-400`"#####, r#####"({
  '--tw-ring-offset-color': "#f87171",
})
;"##### ; "60")]
#[test_case(r#####"tw`ring-offset-red-500`"#####, r#####"({
  '--tw-ring-offset-color': "#ef4444",
})
;"##### ; "61")]
#[test_case(r#####"tw`ring-offset-red-600`"#####, r#####"({
  '--tw-ring-offset-color': "#dc2626",
})
;"##### ; "62")]
#[test_case(r#####"tw`ring-offset-red-700`"#####, r#####"({
  '--tw-ring-offset-color': "#b91c1c",
})
;"##### ; "63")]
#[test_case(r#####"tw`ring-offset-red-800`"#####, r#####"({
  '--tw-ring-offset-color': "#991b1b",
})
;"##### ; "64")]
#[test_case(r#####"tw`ring-offset-red-900`"#####, r#####"({
  '--tw-ring-offset-color': "#7f1d1d",
})
;"##### ; "65")]
#[test_case(r#####"tw`ring-offset-orange-50`"#####, r#####"({
  '--tw-ring-offset-color': "#fff7ed",
})
;"##### ; "66")]
#[test_case(r#####"tw`ring-offset-orange-100`"#####, r#####"({
  '--tw-ring-offset-color': "#ffedd5",
})
;"##### ; "67")]
#[test_case(r#####"tw`ring-offset-orange-200`"#####, r#####"({
  '--tw-ring-offset-color': "#fed7aa",
})
;"##### ; "68")]
#[test_case(r#####"tw`ring-offset-orange-300`"#####, r#####"({
  '--tw-ring-offset-color': "#fdba74",
})
;"##### ; "69")]
#[test_case(r#####"tw`ring-offset-orange-400`"#####, r#####"({
  '--tw-ring-offset-color': "#fb923c",
})
;"##### ; "70")]
#[test_case(r#####"tw`ring-offset-orange-500`"#####, r#####"({
  '--tw-ring-offset-color': "#f97316",
})
;"##### ; "71")]
#[test_case(r#####"tw`ring-offset-orange-600`"#####, r#####"({
  '--tw-ring-offset-color': "#ea580c",
})
;"##### ; "72")]
#[test_case(r#####"tw`ring-offset-orange-700`"#####, r#####"({
  '--tw-ring-offset-color': "#c2410c",
})
;"##### ; "73")]
#[test_case(r#####"tw`ring-offset-orange-800`"#####, r#####"({
  '--tw-ring-offset-color': "#9a3412",
})
;"##### ; "74")]
#[test_case(r#####"tw`ring-offset-orange-900`"#####, r#####"({
  '--tw-ring-offset-color': "#7c2d12",
})
;"##### ; "75")]
#[test_case(r#####"tw`ring-offset-amber-50`"#####, r#####"({
  '--tw-ring-offset-color': "#fffbeb",
})
;"##### ; "76")]
#[test_case(r#####"tw`ring-offset-amber-100`"#####, r#####"({
  '--tw-ring-offset-color': "#fef3c7",
})
;"##### ; "77")]
#[test_case(r#####"tw`ring-offset-amber-200`"#####, r#####"({
  '--tw-ring-offset-color': "#fde68a",
})
;"##### ; "78")]
#[test_case(r#####"tw`ring-offset-amber-300`"#####, r#####"({
  '--tw-ring-offset-color': "#fcd34d",
})
;"##### ; "79")]
#[test_case(r#####"tw`ring-offset-amber-400`"#####, r#####"({
  '--tw-ring-offset-color': "#fbbf24",
})
;"##### ; "80")]
#[test_case(r#####"tw`ring-offset-amber-500`"#####, r#####"({
  '--tw-ring-offset-color': "#f59e0b",
})
;"##### ; "81")]
#[test_case(r#####"tw`ring-offset-amber-600`"#####, r#####"({
  '--tw-ring-offset-color': "#d97706",
})
;"##### ; "82")]
#[test_case(r#####"tw`ring-offset-amber-700`"#####, r#####"({
  '--tw-ring-offset-color': "#b45309",
})
;"##### ; "83")]
#[test_case(r#####"tw`ring-offset-amber-800`"#####, r#####"({
  '--tw-ring-offset-color': "#92400e",
})
;"##### ; "84")]
#[test_case(r#####"tw`ring-offset-amber-900`"#####, r#####"({
  '--tw-ring-offset-color': "#78350f",
})
;"##### ; "85")]
#[test_case(r#####"tw`ring-offset-yellow-50`"#####, r#####"({
  '--tw-ring-offset-color': "#fefce8",
})
;"##### ; "86")]
#[test_case(r#####"tw`ring-offset-yellow-100`"#####, r#####"({
  '--tw-ring-offset-color': "#fef9c3",
})
;"##### ; "87")]
#[test_case(r#####"tw`ring-offset-yellow-200`"#####, r#####"({
  '--tw-ring-offset-color': "#fef08a",
})
;"##### ; "88")]
#[test_case(r#####"tw`ring-offset-yellow-300`"#####, r#####"({
  '--tw-ring-offset-color': "#fde047",
})
;"##### ; "89")]
#[test_case(r#####"tw`ring-offset-yellow-400`"#####, r#####"({
  '--tw-ring-offset-color': "#facc15",
})
;"##### ; "90")]
#[test_case(r#####"tw`ring-offset-yellow-500`"#####, r#####"({
  '--tw-ring-offset-color': "#eab308",
})
;"##### ; "91")]
#[test_case(r#####"tw`ring-offset-yellow-600`"#####, r#####"({
  '--tw-ring-offset-color': "#ca8a04",
})
;"##### ; "92")]
#[test_case(r#####"tw`ring-offset-yellow-700`"#####, r#####"({
  '--tw-ring-offset-color': "#a16207",
})
;"##### ; "93")]
#[test_case(r#####"tw`ring-offset-yellow-800`"#####, r#####"({
  '--tw-ring-offset-color': "#854d0e",
})
;"##### ; "94")]
#[test_case(r#####"tw`ring-offset-yellow-900`"#####, r#####"({
  '--tw-ring-offset-color': "#713f12",
})
;"##### ; "95")]
#[test_case(r#####"tw`ring-offset-lime-50`"#####, r#####"({
  '--tw-ring-offset-color': "#f7fee7",
})
;"##### ; "96")]
#[test_case(r#####"tw`ring-offset-lime-100`"#####, r#####"({
  '--tw-ring-offset-color': "#ecfccb",
})
;"##### ; "97")]
#[test_case(r#####"tw`ring-offset-lime-200`"#####, r#####"({
  '--tw-ring-offset-color': "#d9f99d",
})
;"##### ; "98")]
#[test_case(r#####"tw`ring-offset-lime-300`"#####, r#####"({
  '--tw-ring-offset-color': "#bef264",
})
;"##### ; "99")]
#[test_case(r#####"tw`ring-offset-lime-400`"#####, r#####"({
  '--tw-ring-offset-color': "#a3e635",
})
;"##### ; "100")]
#[test_case(r#####"tw`ring-offset-lime-500`"#####, r#####"({
  '--tw-ring-offset-color': "#84cc16",
})
;"##### ; "101")]
#[test_case(r#####"tw`ring-offset-lime-600`"#####, r#####"({
  '--tw-ring-offset-color': "#65a30d",
})
;"##### ; "102")]
#[test_case(r#####"tw`ring-offset-lime-700`"#####, r#####"({
  '--tw-ring-offset-color': "#4d7c0f",
})
;"##### ; "103")]
#[test_case(r#####"tw`ring-offset-lime-800`"#####, r#####"({
  '--tw-ring-offset-color': "#3f6212",
})
;"##### ; "104")]
#[test_case(r#####"tw`ring-offset-lime-900`"#####, r#####"({
  '--tw-ring-offset-color': "#365314",
})
;"##### ; "105")]
#[test_case(r#####"tw`ring-offset-green-50`"#####, r#####"({
  '--tw-ring-offset-color': "#f0fdf4",
})
;"##### ; "106")]
#[test_case(r#####"tw`ring-offset-green-100`"#####, r#####"({
  '--tw-ring-offset-color': "#dcfce7",
})
;"##### ; "107")]
#[test_case(r#####"tw`ring-offset-green-200`"#####, r#####"({
  '--tw-ring-offset-color': "#bbf7d0",
})
;"##### ; "108")]
#[test_case(r#####"tw`ring-offset-green-300`"#####, r#####"({
  '--tw-ring-offset-color': "#86efac",
})
;"##### ; "109")]
#[test_case(r#####"tw`ring-offset-green-400`"#####, r#####"({
  '--tw-ring-offset-color': "#4ade80",
})
;"##### ; "110")]
#[test_case(r#####"tw`ring-offset-green-500`"#####, r#####"({
  '--tw-ring-offset-color': "#22c55e",
})
;"##### ; "111")]
#[test_case(r#####"tw`ring-offset-green-600`"#####, r#####"({
  '--tw-ring-offset-color': "#16a34a",
})
;"##### ; "112")]
#[test_case(r#####"tw`ring-offset-green-700`"#####, r#####"({
  '--tw-ring-offset-color': "#15803d",
})
;"##### ; "113")]
#[test_case(r#####"tw`ring-offset-green-800`"#####, r#####"({
  '--tw-ring-offset-color': "#166534",
})
;"##### ; "114")]
#[test_case(r#####"tw`ring-offset-green-900`"#####, r#####"({
  '--tw-ring-offset-color': "#14532d",
})
;"##### ; "115")]
#[test_case(r#####"tw`ring-offset-emerald-50`"#####, r#####"({
  '--tw-ring-offset-color': "#ecfdf5",
})
;"##### ; "116")]
#[test_case(r#####"tw`ring-offset-emerald-100`"#####, r#####"({
  '--tw-ring-offset-color': "#d1fae5",
})
;"##### ; "117")]
#[test_case(r#####"tw`ring-offset-emerald-200`"#####, r#####"({
  '--tw-ring-offset-color': "#a7f3d0",
})
;"##### ; "118")]
#[test_case(r#####"tw`ring-offset-emerald-300`"#####, r#####"({
  '--tw-ring-offset-color': "#6ee7b7",
})
;"##### ; "119")]
#[test_case(r#####"tw`ring-offset-emerald-400`"#####, r#####"({
  '--tw-ring-offset-color': "#34d399",
})
;"##### ; "120")]
#[test_case(r#####"tw`ring-offset-emerald-500`"#####, r#####"({
  '--tw-ring-offset-color': "#10b981",
})
;"##### ; "121")]
#[test_case(r#####"tw`ring-offset-emerald-600`"#####, r#####"({
  '--tw-ring-offset-color': "#059669",
})
;"##### ; "122")]
#[test_case(r#####"tw`ring-offset-emerald-700`"#####, r#####"({
  '--tw-ring-offset-color': "#047857",
})
;"##### ; "123")]
#[test_case(r#####"tw`ring-offset-emerald-800`"#####, r#####"({
  '--tw-ring-offset-color': "#065f46",
})
;"##### ; "124")]
#[test_case(r#####"tw`ring-offset-emerald-900`"#####, r#####"({
  '--tw-ring-offset-color': "#064e3b",
})
;"##### ; "125")]
#[test_case(r#####"tw`ring-offset-teal-50`"#####, r#####"({
  '--tw-ring-offset-color': "#f0fdfa",
})
;"##### ; "126")]
#[test_case(r#####"tw`ring-offset-teal-100`"#####, r#####"({
  '--tw-ring-offset-color': "#ccfbf1",
})
;"##### ; "127")]
#[test_case(r#####"tw`ring-offset-teal-200`"#####, r#####"({
  '--tw-ring-offset-color': "#99f6e4",
})
;"##### ; "128")]
#[test_case(r#####"tw`ring-offset-teal-300`"#####, r#####"({
  '--tw-ring-offset-color': "#5eead4",
})
;"##### ; "129")]
#[test_case(r#####"tw`ring-offset-teal-400`"#####, r#####"({
  '--tw-ring-offset-color': "#2dd4bf",
})
;"##### ; "130")]
#[test_case(r#####"tw`ring-offset-teal-500`"#####, r#####"({
  '--tw-ring-offset-color': "#14b8a6",
})
;"##### ; "131")]
#[test_case(r#####"tw`ring-offset-teal-600`"#####, r#####"({
  '--tw-ring-offset-color': "#0d9488",
})
;"##### ; "132")]
#[test_case(r#####"tw`ring-offset-teal-700`"#####, r#####"({
  '--tw-ring-offset-color': "#0f766e",
})
;"##### ; "133")]
#[test_case(r#####"tw`ring-offset-teal-800`"#####, r#####"({
  '--tw-ring-offset-color': "#115e59",
})
;"##### ; "134")]
#[test_case(r#####"tw`ring-offset-teal-900`"#####, r#####"({
  '--tw-ring-offset-color': "#134e4a",
})
;"##### ; "135")]
#[test_case(r#####"tw`ring-offset-cyan-50`"#####, r#####"({
  '--tw-ring-offset-color': "#ecfeff",
})
;"##### ; "136")]
#[test_case(r#####"tw`ring-offset-cyan-100`"#####, r#####"({
  '--tw-ring-offset-color': "#cffafe",
})
;"##### ; "137")]
#[test_case(r#####"tw`ring-offset-cyan-200`"#####, r#####"({
  '--tw-ring-offset-color': "#a5f3fc",
})
;"##### ; "138")]
#[test_case(r#####"tw`ring-offset-cyan-300`"#####, r#####"({
  '--tw-ring-offset-color': "#67e8f9",
})
;"##### ; "139")]
#[test_case(r#####"tw`ring-offset-cyan-400`"#####, r#####"({
  '--tw-ring-offset-color': "#22d3ee",
})
;"##### ; "140")]
#[test_case(r#####"tw`ring-offset-cyan-500`"#####, r#####"({
  '--tw-ring-offset-color': "#06b6d4",
})
;"##### ; "141")]
#[test_case(r#####"tw`ring-offset-cyan-600`"#####, r#####"({
  '--tw-ring-offset-color': "#0891b2",
})
;"##### ; "142")]
#[test_case(r#####"tw`ring-offset-cyan-700`"#####, r#####"({
  '--tw-ring-offset-color': "#0e7490",
})
;"##### ; "143")]
#[test_case(r#####"tw`ring-offset-cyan-800`"#####, r#####"({
  '--tw-ring-offset-color': "#155e75",
})
;"##### ; "144")]
#[test_case(r#####"tw`ring-offset-cyan-900`"#####, r#####"({
  '--tw-ring-offset-color': "#164e63",
})
;"##### ; "145")]
#[test_case(r#####"tw`ring-offset-sky-50`"#####, r#####"({
  '--tw-ring-offset-color': "#f0f9ff",
})
;"##### ; "146")]
#[test_case(r#####"tw`ring-offset-sky-100`"#####, r#####"({
  '--tw-ring-offset-color': "#e0f2fe",
})
;"##### ; "147")]
#[test_case(r#####"tw`ring-offset-sky-200`"#####, r#####"({
  '--tw-ring-offset-color': "#bae6fd",
})
;"##### ; "148")]
#[test_case(r#####"tw`ring-offset-sky-300`"#####, r#####"({
  '--tw-ring-offset-color': "#7dd3fc",
})
;"##### ; "149")]
#[test_case(r#####"tw`ring-offset-sky-400`"#####, r#####"({
  '--tw-ring-offset-color': "#38bdf8",
})
;"##### ; "150")]
#[test_case(r#####"tw`ring-offset-sky-500`"#####, r#####"({
  '--tw-ring-offset-color': "#0ea5e9",
})
;"##### ; "151")]
#[test_case(r#####"tw`ring-offset-sky-600`"#####, r#####"({
  '--tw-ring-offset-color': "#0284c7",
})
;"##### ; "152")]
#[test_case(r#####"tw`ring-offset-sky-700`"#####, r#####"({
  '--tw-ring-offset-color': "#0369a1",
})
;"##### ; "153")]
#[test_case(r#####"tw`ring-offset-sky-800`"#####, r#####"({
  '--tw-ring-offset-color': "#075985",
})
;"##### ; "154")]
#[test_case(r#####"tw`ring-offset-sky-900`"#####, r#####"({
  '--tw-ring-offset-color': "#0c4a6e",
})
;"##### ; "155")]
#[test_case(r#####"tw`ring-offset-blue-50`"#####, r#####"({
  '--tw-ring-offset-color': "#eff6ff",
})
;"##### ; "156")]
#[test_case(r#####"tw`ring-offset-blue-100`"#####, r#####"({
  '--tw-ring-offset-color': "#dbeafe",
})
;"##### ; "157")]
#[test_case(r#####"tw`ring-offset-blue-200`"#####, r#####"({
  '--tw-ring-offset-color': "#bfdbfe",
})
;"##### ; "158")]
#[test_case(r#####"tw`ring-offset-blue-300`"#####, r#####"({
  '--tw-ring-offset-color': "#93c5fd",
})
;"##### ; "159")]
#[test_case(r#####"tw`ring-offset-blue-400`"#####, r#####"({
  '--tw-ring-offset-color': "#60a5fa",
})
;"##### ; "160")]
#[test_case(r#####"tw`ring-offset-blue-500`"#####, r#####"({
  '--tw-ring-offset-color': "#3b82f6",
})
;"##### ; "161")]
#[test_case(r#####"tw`ring-offset-blue-600`"#####, r#####"({
  '--tw-ring-offset-color': "#2563eb",
})
;"##### ; "162")]
#[test_case(r#####"tw`ring-offset-blue-700`"#####, r#####"({
  '--tw-ring-offset-color': "#1d4ed8",
})
;"##### ; "163")]
#[test_case(r#####"tw`ring-offset-blue-800`"#####, r#####"({
  '--tw-ring-offset-color': "#1e40af",
})
;"##### ; "164")]
#[test_case(r#####"tw`ring-offset-blue-900`"#####, r#####"({
  '--tw-ring-offset-color': "#1e3a8a",
})
;"##### ; "165")]
#[test_case(r#####"tw`ring-offset-indigo-50`"#####, r#####"({
  '--tw-ring-offset-color': "#eef2ff",
})
;"##### ; "166")]
#[test_case(r#####"tw`ring-offset-indigo-100`"#####, r#####"({
  '--tw-ring-offset-color': "#e0e7ff",
})
;"##### ; "167")]
#[test_case(r#####"tw`ring-offset-indigo-200`"#####, r#####"({
  '--tw-ring-offset-color': "#c7d2fe",
})
;"##### ; "168")]
#[test_case(r#####"tw`ring-offset-indigo-300`"#####, r#####"({
  '--tw-ring-offset-color': "#a5b4fc",
})
;"##### ; "169")]
#[test_case(r#####"tw`ring-offset-indigo-400`"#####, r#####"({
  '--tw-ring-offset-color': "#818cf8",
})
;"##### ; "170")]
#[test_case(r#####"tw`ring-offset-indigo-500`"#####, r#####"({
  '--tw-ring-offset-color': "#6366f1",
})
;"##### ; "171")]
#[test_case(r#####"tw`ring-offset-indigo-600`"#####, r#####"({
  '--tw-ring-offset-color': "#4f46e5",
})
;"##### ; "172")]
#[test_case(r#####"tw`ring-offset-indigo-700`"#####, r#####"({
  '--tw-ring-offset-color': "#4338ca",
})
;"##### ; "173")]
#[test_case(r#####"tw`ring-offset-indigo-800`"#####, r#####"({
  '--tw-ring-offset-color': "#3730a3",
})
;"##### ; "174")]
#[test_case(r#####"tw`ring-offset-indigo-900`"#####, r#####"({
  '--tw-ring-offset-color': "#312e81",
})
;"##### ; "175")]
#[test_case(r#####"tw`ring-offset-violet-50`"#####, r#####"({
  '--tw-ring-offset-color': "#f5f3ff",
})
;"##### ; "176")]
#[test_case(r#####"tw`ring-offset-violet-100`"#####, r#####"({
  '--tw-ring-offset-color': "#ede9fe",
})
;"##### ; "177")]
#[test_case(r#####"tw`ring-offset-violet-200`"#####, r#####"({
  '--tw-ring-offset-color': "#ddd6fe",
})
;"##### ; "178")]
#[test_case(r#####"tw`ring-offset-violet-300`"#####, r#####"({
  '--tw-ring-offset-color': "#c4b5fd",
})
;"##### ; "179")]
#[test_case(r#####"tw`ring-offset-violet-400`"#####, r#####"({
  '--tw-ring-offset-color': "#a78bfa",
})
;"##### ; "180")]
#[test_case(r#####"tw`ring-offset-violet-500`"#####, r#####"({
  '--tw-ring-offset-color': "#8b5cf6",
})
;"##### ; "181")]
#[test_case(r#####"tw`ring-offset-violet-600`"#####, r#####"({
  '--tw-ring-offset-color': "#7c3aed",
})
;"##### ; "182")]
#[test_case(r#####"tw`ring-offset-violet-700`"#####, r#####"({
  '--tw-ring-offset-color': "#6d28d9",
})
;"##### ; "183")]
#[test_case(r#####"tw`ring-offset-violet-800`"#####, r#####"({
  '--tw-ring-offset-color': "#5b21b6",
})
;"##### ; "184")]
#[test_case(r#####"tw`ring-offset-violet-900`"#####, r#####"({
  '--tw-ring-offset-color': "#4c1d95",
})
;"##### ; "185")]
#[test_case(r#####"tw`ring-offset-purple-50`"#####, r#####"({
  '--tw-ring-offset-color': "#faf5ff",
})
;"##### ; "186")]
#[test_case(r#####"tw`ring-offset-purple-100`"#####, r#####"({
  '--tw-ring-offset-color': "#f3e8ff",
})
;"##### ; "187")]
#[test_case(r#####"tw`ring-offset-purple-200`"#####, r#####"({
  '--tw-ring-offset-color': "#e9d5ff",
})
;"##### ; "188")]
#[test_case(r#####"tw`ring-offset-purple-300`"#####, r#####"({
  '--tw-ring-offset-color': "#d8b4fe",
})
;"##### ; "189")]
#[test_case(r#####"tw`ring-offset-purple-400`"#####, r#####"({
  '--tw-ring-offset-color': "#c084fc",
})
;"##### ; "190")]
#[test_case(r#####"tw`ring-offset-purple-500`"#####, r#####"({
  '--tw-ring-offset-color': "#a855f7",
})
;"##### ; "191")]
#[test_case(r#####"tw`ring-offset-purple-600`"#####, r#####"({
  '--tw-ring-offset-color': "#9333ea",
})
;"##### ; "192")]
#[test_case(r#####"tw`ring-offset-purple-700`"#####, r#####"({
  '--tw-ring-offset-color': "#7e22ce",
})
;"##### ; "193")]
#[test_case(r#####"tw`ring-offset-purple-800`"#####, r#####"({
  '--tw-ring-offset-color': "#6b21a8",
})
;"##### ; "194")]
#[test_case(r#####"tw`ring-offset-purple-900`"#####, r#####"({
  '--tw-ring-offset-color': "#581c87",
})
;"##### ; "195")]
#[test_case(r#####"tw`ring-offset-fuchsia-50`"#####, r#####"({
  '--tw-ring-offset-color': "#fdf4ff",
})
;"##### ; "196")]
#[test_case(r#####"tw`ring-offset-fuchsia-100`"#####, r#####"({
  '--tw-ring-offset-color': "#fae8ff",
})
;"##### ; "197")]
#[test_case(r#####"tw`ring-offset-fuchsia-200`"#####, r#####"({
  '--tw-ring-offset-color': "#f5d0fe",
})
;"##### ; "198")]
#[test_case(r#####"tw`ring-offset-fuchsia-300`"#####, r#####"({
  '--tw-ring-offset-color': "#f0abfc",
})
;"##### ; "199")]
#[test_case(r#####"tw`ring-offset-fuchsia-400`"#####, r#####"({
  '--tw-ring-offset-color': "#e879f9",
})
;"##### ; "200")]
#[test_case(r#####"tw`ring-offset-fuchsia-500`"#####, r#####"({
  '--tw-ring-offset-color': "#d946ef",
})
;"##### ; "201")]
#[test_case(r#####"tw`ring-offset-fuchsia-600`"#####, r#####"({
  '--tw-ring-offset-color': "#c026d3",
})
;"##### ; "202")]
#[test_case(r#####"tw`ring-offset-fuchsia-700`"#####, r#####"({
  '--tw-ring-offset-color': "#a21caf",
})
;"##### ; "203")]
#[test_case(r#####"tw`ring-offset-fuchsia-800`"#####, r#####"({
  '--tw-ring-offset-color': "#86198f",
})
;"##### ; "204")]
#[test_case(r#####"tw`ring-offset-fuchsia-900`"#####, r#####"({
  '--tw-ring-offset-color': "#701a75",
})
;"##### ; "205")]
#[test_case(r#####"tw`ring-offset-pink-50`"#####, r#####"({
  '--tw-ring-offset-color': "#fdf2f8",
})
;"##### ; "206")]
#[test_case(r#####"tw`ring-offset-pink-100`"#####, r#####"({
  '--tw-ring-offset-color': "#fce7f3",
})
;"##### ; "207")]
#[test_case(r#####"tw`ring-offset-pink-200`"#####, r#####"({
  '--tw-ring-offset-color': "#fbcfe8",
})
;"##### ; "208")]
#[test_case(r#####"tw`ring-offset-pink-300`"#####, r#####"({
  '--tw-ring-offset-color': "#f9a8d4",
})
;"##### ; "209")]
#[test_case(r#####"tw`ring-offset-pink-400`"#####, r#####"({
  '--tw-ring-offset-color': "#f472b6",
})
;"##### ; "210")]
#[test_case(r#####"tw`ring-offset-pink-500`"#####, r#####"({
  '--tw-ring-offset-color': "#ec4899",
})
;"##### ; "211")]
#[test_case(r#####"tw`ring-offset-pink-600`"#####, r#####"({
  '--tw-ring-offset-color': "#db2777",
})
;"##### ; "212")]
#[test_case(r#####"tw`ring-offset-pink-700`"#####, r#####"({
  '--tw-ring-offset-color': "#be185d",
})
;"##### ; "213")]
#[test_case(r#####"tw`ring-offset-pink-800`"#####, r#####"({
  '--tw-ring-offset-color': "#9d174d",
})
;"##### ; "214")]
#[test_case(r#####"tw`ring-offset-pink-900`"#####, r#####"({
  '--tw-ring-offset-color': "#831843",
})
;"##### ; "215")]
#[test_case(r#####"tw`ring-offset-rose-50`"#####, r#####"({
  '--tw-ring-offset-color': "#fff1f2",
})
;"##### ; "216")]
#[test_case(r#####"tw`ring-offset-rose-100`"#####, r#####"({
  '--tw-ring-offset-color': "#ffe4e6",
})
;"##### ; "217")]
#[test_case(r#####"tw`ring-offset-rose-200`"#####, r#####"({
  '--tw-ring-offset-color': "#fecdd3",
})
;"##### ; "218")]
#[test_case(r#####"tw`ring-offset-rose-300`"#####, r#####"({
  '--tw-ring-offset-color': "#fda4af",
})
;"##### ; "219")]
#[test_case(r#####"tw`ring-offset-rose-400`"#####, r#####"({
  '--tw-ring-offset-color': "#fb7185",
})
;"##### ; "220")]
#[test_case(r#####"tw`ring-offset-rose-500`"#####, r#####"({
  '--tw-ring-offset-color': "#f43f5e",
})
;"##### ; "221")]
#[test_case(r#####"tw`ring-offset-rose-600`"#####, r#####"({
  '--tw-ring-offset-color': "#e11d48",
})
;"##### ; "222")]
#[test_case(r#####"tw`ring-offset-rose-700`"#####, r#####"({
  '--tw-ring-offset-color': "#be123c",
})
;"##### ; "223")]
#[test_case(r#####"tw`ring-offset-rose-800`"#####, r#####"({
  '--tw-ring-offset-color': "#9f1239",
})
;"##### ; "224")]
#[test_case(r#####"tw`ring-offset-rose-900`"#####, r#####"({
  '--tw-ring-offset-color': "#881337",
})
;"##### ; "225")]
#[test_case(r#####"tw`ring-offset-rose-900/50`"#####, r#####"({
  '--tw-ring-offset-color': "rgb(136 19 55 / 0.5)",
})
;"##### ; "226")]
#[test_case(r#####"tw`ring-offset-rose-900/[.50]`"#####, r#####"({
  '--tw-ring-offset-color': "rgb(136 19 55 / .50)",
})
;"##### ; "227")]
#[test_case(r#####"tw`ring-offset-[#50d71e]`"#####, r#####"({
  '--tw-ring-offset-color': "#50d71e",
})
;"##### ; "228")]
#[test_case(r#####"tw`ring-offset-red-500`"#####, r#####"({
  '--tw-ring-offset-color': "#ef4444",
})
;"##### ; "229")]
#[test_case(r#####"tw`ring-offset-red-500/25`"#####, r#####"({
  '--tw-ring-offset-color': "rgb(239 68 68 / 0.25)",
})
;"##### ; "230")]
#[test_case(r#####"tw`ring-offset-red-500/fromConfig`"#####, r#####"({
  '--tw-ring-offset-color': "#000",
})
;"##### ; "231")]
#[test_case(r#####"tw`ring-offset-red-500/fromConfig/25`"#####, r#####"({
  '--tw-ring-offset-color': "rgb(0 0 0 / 0.25)",
})
;"##### ; "232")]
#[test_case(r#####"tw`ring-offset-red-500/fromConfig/[.555]`"#####, r#####"({
  '--tw-ring-offset-color': "rgb(0 0 0 / .555)",
})
;"##### ; "233")]
#[test_case(r#####"tw`ring-offset-red-500/fromConfig/[var(--myvar)]`"#####, r#####"({
  '--tw-ring-offset-color': "rgb(0 0 0 / var(--myvar))",
})
;"##### ; "234")]
#[test_case(r#####"tw`ring-offset-red-500/[.555]`"#####, r#####"({
  '--tw-ring-offset-color': "rgb(239 68 68 / .555)",
})
;"##### ; "235")]
#[test_case(r#####"tw`ring-offset-red-500/[var(--myvar)]`"#####, r#####"({
  '--tw-ring-offset-color': "rgb(239 68 68 / var(--myvar))",
})
;"##### ; "236")]
#[test_case(r#####"tw`ring-offset-[theme('colors.red.500')]`"#####, r#####"({
  '--tw-ring-offset-color': "#ef4444",
})
;"##### ; "237")]
#[test_case(r#####"tw`ring-offset-[theme('colors.red.500')]/20`"#####, r#####"({
  '--tw-ring-offset-color': "rgb(239 68 68 / 0.2)",
})
;"##### ; "238")]
#[test_case(r#####"tw`ring-offset-electric`"#####, r#####"({
  '--tw-ring-offset-color': "rgb(219, 0, 255)",
})
;"##### ; "239")]
#[test_case(r#####"tw`ring-offset-electric/25`"#####, r#####"({
  '--tw-ring-offset-color': "rgba(219, 0, 255, 0.25)",
})
;"##### ; "240")]
#[test_case(r#####"tw`ring-offset-electric/[.555]`"#####, r#####"({
  '--tw-ring-offset-color': "rgba(219, 0, 255, .555)",
})
;"##### ; "241")]
#[test_case(r#####"tw`ring-offset-electric/[var(--myvar)]`"#####, r#####"({
  '--tw-ring-offset-color': "rgba(219, 0, 255, var(--myvar))",
})
;"##### ; "242")]
#[test_case(r#####"tw`ring-offset-[theme('colors.electric')]`"#####, r#####"({
  '--tw-ring-offset-color': "rgb(219, 0, 255)",
})
;"##### ; "243")]
#[test_case(r#####"tw`ring-offset-[theme('colors.electric')]/20`"#####, r#####"({
  '--tw-ring-offset-color': "rgb(219 0 255 / 0.2)",
})
;"##### ; "244")]
#[test_case(r#####"tw`ring-offset-[color:green]`"#####, r#####"({
  '--tw-ring-offset-color': "green",
})
;"##### ; "245")]
#[test_case(r#####"tw`ring-offset-[color:rgba(255, 255, 255, .45)]`"#####, r#####"({
  '--tw-ring-offset-color': "rgba(255, 255, 255, .45)",
})
;"##### ; "246")]
#[test_case(r#####"tw`ring-offset-[length:10px]`"#####, r#####"({
  '--tw-ring-offset-width': "10px",
})"##### ; "247")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
