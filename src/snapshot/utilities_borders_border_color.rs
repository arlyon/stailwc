use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`borderColor.`"#####, r#####"({
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
  DEFAULT: "#e5e7eb",
})
;"##### ; "0")]
#[test_case(r#####"tw`border-inherit`"#####, r#####"({
  borderColor: "inherit",
})
;"##### ; "1")]
#[test_case(r#####"tw`border-current`"#####, r#####"({
  borderColor: "currentColor",
})
;"##### ; "2")]
#[test_case(r#####"tw`border-transparent`"#####, r#####"({
  borderColor: "transparent",
})
;"##### ; "3")]
#[test_case(r#####"tw`border-black`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(0 0 0 / var(--tw-border-opacity))",
})
;"##### ; "4")]
#[test_case(r#####"tw`border-white`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(255 255 255 / var(--tw-border-opacity))",
})
;"##### ; "5")]
#[test_case(r#####"tw`border-slate-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(248 250 252 / var(--tw-border-opacity))",
})
;"##### ; "6")]
#[test_case(r#####"tw`border-slate-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(241 245 249 / var(--tw-border-opacity))",
})
;"##### ; "7")]
#[test_case(r#####"tw`border-slate-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(226 232 240 / var(--tw-border-opacity))",
})
;"##### ; "8")]
#[test_case(r#####"tw`border-slate-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(203 213 225 / var(--tw-border-opacity))",
})
;"##### ; "9")]
#[test_case(r#####"tw`border-slate-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(148 163 184 / var(--tw-border-opacity))",
})
;"##### ; "10")]
#[test_case(r#####"tw`border-slate-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(100 116 139 / var(--tw-border-opacity))",
})
;"##### ; "11")]
#[test_case(r#####"tw`border-slate-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(71 85 105 / var(--tw-border-opacity))",
})
;"##### ; "12")]
#[test_case(r#####"tw`border-slate-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(51 65 85 / var(--tw-border-opacity))",
})
;"##### ; "13")]
#[test_case(r#####"tw`border-slate-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(30 41 59 / var(--tw-border-opacity))",
})
;"##### ; "14")]
#[test_case(r#####"tw`border-slate-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(15 23 42 / var(--tw-border-opacity))",
})
;"##### ; "15")]
#[test_case(r#####"tw`border-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "16")]
#[test_case(r#####"tw`border-gray-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(243 244 246 / var(--tw-border-opacity))",
})
;"##### ; "17")]
#[test_case(r#####"tw`border-gray-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(229 231 235 / var(--tw-border-opacity))",
})
;"##### ; "18")]
#[test_case(r#####"tw`border-gray-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(209 213 219 / var(--tw-border-opacity))",
})
;"##### ; "19")]
#[test_case(r#####"tw`border-gray-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(156 163 175 / var(--tw-border-opacity))",
})
;"##### ; "20")]
#[test_case(r#####"tw`border-gray-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(107 114 128 / var(--tw-border-opacity))",
})
;"##### ; "21")]
#[test_case(r#####"tw`border-gray-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(75 85 99 / var(--tw-border-opacity))",
})
;"##### ; "22")]
#[test_case(r#####"tw`border-gray-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(55 65 81 / var(--tw-border-opacity))",
})
;"##### ; "23")]
#[test_case(r#####"tw`border-gray-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(31 41 55 / var(--tw-border-opacity))",
})
;"##### ; "24")]
#[test_case(r#####"tw`border-gray-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(17 24 39 / var(--tw-border-opacity))",
})
;"##### ; "25")]
#[test_case(r#####"tw`border-zinc-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "26")]
#[test_case(r#####"tw`border-zinc-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(244 244 245 / var(--tw-border-opacity))",
})
;"##### ; "27")]
#[test_case(r#####"tw`border-zinc-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(228 228 231 / var(--tw-border-opacity))",
})
;"##### ; "28")]
#[test_case(r#####"tw`border-zinc-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(212 212 216 / var(--tw-border-opacity))",
})
;"##### ; "29")]
#[test_case(r#####"tw`border-zinc-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(161 161 170 / var(--tw-border-opacity))",
})
;"##### ; "30")]
#[test_case(r#####"tw`border-zinc-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(113 113 122 / var(--tw-border-opacity))",
})
;"##### ; "31")]
#[test_case(r#####"tw`border-zinc-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(82 82 91 / var(--tw-border-opacity))",
})
;"##### ; "32")]
#[test_case(r#####"tw`border-zinc-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(63 63 70 / var(--tw-border-opacity))",
})
;"##### ; "33")]
#[test_case(r#####"tw`border-zinc-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(39 39 42 / var(--tw-border-opacity))",
})
;"##### ; "34")]
#[test_case(r#####"tw`border-zinc-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(24 24 27 / var(--tw-border-opacity))",
})
;"##### ; "35")]
#[test_case(r#####"tw`border-neutral-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "36")]
#[test_case(r#####"tw`border-neutral-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(245 245 245 / var(--tw-border-opacity))",
})
;"##### ; "37")]
#[test_case(r#####"tw`border-neutral-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(229 229 229 / var(--tw-border-opacity))",
})
;"##### ; "38")]
#[test_case(r#####"tw`border-neutral-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(212 212 212 / var(--tw-border-opacity))",
})
;"##### ; "39")]
#[test_case(r#####"tw`border-neutral-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(163 163 163 / var(--tw-border-opacity))",
})
;"##### ; "40")]
#[test_case(r#####"tw`border-neutral-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(115 115 115 / var(--tw-border-opacity))",
})
;"##### ; "41")]
#[test_case(r#####"tw`border-neutral-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(82 82 82 / var(--tw-border-opacity))",
})
;"##### ; "42")]
#[test_case(r#####"tw`border-neutral-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(64 64 64 / var(--tw-border-opacity))",
})
;"##### ; "43")]
#[test_case(r#####"tw`border-neutral-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(38 38 38 / var(--tw-border-opacity))",
})
;"##### ; "44")]
#[test_case(r#####"tw`border-neutral-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(23 23 23 / var(--tw-border-opacity))",
})
;"##### ; "45")]
#[test_case(r#####"tw`border-stone-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(250 250 249 / var(--tw-border-opacity))",
})
;"##### ; "46")]
#[test_case(r#####"tw`border-stone-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(245 245 244 / var(--tw-border-opacity))",
})
;"##### ; "47")]
#[test_case(r#####"tw`border-stone-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(231 229 228 / var(--tw-border-opacity))",
})
;"##### ; "48")]
#[test_case(r#####"tw`border-stone-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(214 211 209 / var(--tw-border-opacity))",
})
;"##### ; "49")]
#[test_case(r#####"tw`border-stone-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(168 162 158 / var(--tw-border-opacity))",
})
;"##### ; "50")]
#[test_case(r#####"tw`border-stone-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(120 113 108 / var(--tw-border-opacity))",
})
;"##### ; "51")]
#[test_case(r#####"tw`border-stone-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(87 83 78 / var(--tw-border-opacity))",
})
;"##### ; "52")]
#[test_case(r#####"tw`border-stone-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(68 64 60 / var(--tw-border-opacity))",
})
;"##### ; "53")]
#[test_case(r#####"tw`border-stone-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(41 37 36 / var(--tw-border-opacity))",
})
;"##### ; "54")]
#[test_case(r#####"tw`border-stone-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(28 25 23 / var(--tw-border-opacity))",
})
;"##### ; "55")]
#[test_case(r#####"tw`border-red-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(254 242 242 / var(--tw-border-opacity))",
})
;"##### ; "56")]
#[test_case(r#####"tw`border-red-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(254 226 226 / var(--tw-border-opacity))",
})
;"##### ; "57")]
#[test_case(r#####"tw`border-red-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(254 202 202 / var(--tw-border-opacity))",
})
;"##### ; "58")]
#[test_case(r#####"tw`border-red-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(252 165 165 / var(--tw-border-opacity))",
})
;"##### ; "59")]
#[test_case(r#####"tw`border-red-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(248 113 113 / var(--tw-border-opacity))",
})
;"##### ; "60")]
#[test_case(r#####"tw`border-red-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(239 68 68 / var(--tw-border-opacity))",
})
;"##### ; "61")]
#[test_case(r#####"tw`border-red-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(220 38 38 / var(--tw-border-opacity))",
})
;"##### ; "62")]
#[test_case(r#####"tw`border-red-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(185 28 28 / var(--tw-border-opacity))",
})
;"##### ; "63")]
#[test_case(r#####"tw`border-red-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(153 27 27 / var(--tw-border-opacity))",
})
;"##### ; "64")]
#[test_case(r#####"tw`border-red-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(127 29 29 / var(--tw-border-opacity))",
})
;"##### ; "65")]
#[test_case(r#####"tw`border-orange-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(255 247 237 / var(--tw-border-opacity))",
})
;"##### ; "66")]
#[test_case(r#####"tw`border-orange-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(255 237 213 / var(--tw-border-opacity))",
})
;"##### ; "67")]
#[test_case(r#####"tw`border-orange-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(254 215 170 / var(--tw-border-opacity))",
})
;"##### ; "68")]
#[test_case(r#####"tw`border-orange-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(253 186 116 / var(--tw-border-opacity))",
})
;"##### ; "69")]
#[test_case(r#####"tw`border-orange-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(251 146 60 / var(--tw-border-opacity))",
})
;"##### ; "70")]
#[test_case(r#####"tw`border-orange-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(249 115 22 / var(--tw-border-opacity))",
})
;"##### ; "71")]
#[test_case(r#####"tw`border-orange-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(234 88 12 / var(--tw-border-opacity))",
})
;"##### ; "72")]
#[test_case(r#####"tw`border-orange-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(194 65 12 / var(--tw-border-opacity))",
})
;"##### ; "73")]
#[test_case(r#####"tw`border-orange-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(154 52 18 / var(--tw-border-opacity))",
})
;"##### ; "74")]
#[test_case(r#####"tw`border-orange-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(124 45 18 / var(--tw-border-opacity))",
})
;"##### ; "75")]
#[test_case(r#####"tw`border-amber-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(255 251 235 / var(--tw-border-opacity))",
})
;"##### ; "76")]
#[test_case(r#####"tw`border-amber-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(254 243 199 / var(--tw-border-opacity))",
})
;"##### ; "77")]
#[test_case(r#####"tw`border-amber-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(253 230 138 / var(--tw-border-opacity))",
})
;"##### ; "78")]
#[test_case(r#####"tw`border-amber-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(252 211 77 / var(--tw-border-opacity))",
})
;"##### ; "79")]
#[test_case(r#####"tw`border-amber-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(251 191 36 / var(--tw-border-opacity))",
})
;"##### ; "80")]
#[test_case(r#####"tw`border-amber-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(245 158 11 / var(--tw-border-opacity))",
})
;"##### ; "81")]
#[test_case(r#####"tw`border-amber-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(217 119 6 / var(--tw-border-opacity))",
})
;"##### ; "82")]
#[test_case(r#####"tw`border-amber-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(180 83 9 / var(--tw-border-opacity))",
})
;"##### ; "83")]
#[test_case(r#####"tw`border-amber-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(146 64 14 / var(--tw-border-opacity))",
})
;"##### ; "84")]
#[test_case(r#####"tw`border-amber-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(120 53 15 / var(--tw-border-opacity))",
})
;"##### ; "85")]
#[test_case(r#####"tw`border-yellow-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(254 252 232 / var(--tw-border-opacity))",
})
;"##### ; "86")]
#[test_case(r#####"tw`border-yellow-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(254 249 195 / var(--tw-border-opacity))",
})
;"##### ; "87")]
#[test_case(r#####"tw`border-yellow-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(254 240 138 / var(--tw-border-opacity))",
})
;"##### ; "88")]
#[test_case(r#####"tw`border-yellow-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(253 224 71 / var(--tw-border-opacity))",
})
;"##### ; "89")]
#[test_case(r#####"tw`border-yellow-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(250 204 21 / var(--tw-border-opacity))",
})
;"##### ; "90")]
#[test_case(r#####"tw`border-yellow-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(234 179 8 / var(--tw-border-opacity))",
})
;"##### ; "91")]
#[test_case(r#####"tw`border-yellow-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(202 138 4 / var(--tw-border-opacity))",
})
;"##### ; "92")]
#[test_case(r#####"tw`border-yellow-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(161 98 7 / var(--tw-border-opacity))",
})
;"##### ; "93")]
#[test_case(r#####"tw`border-yellow-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(133 77 14 / var(--tw-border-opacity))",
})
;"##### ; "94")]
#[test_case(r#####"tw`border-yellow-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(113 63 18 / var(--tw-border-opacity))",
})
;"##### ; "95")]
#[test_case(r#####"tw`border-lime-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(247 254 231 / var(--tw-border-opacity))",
})
;"##### ; "96")]
#[test_case(r#####"tw`border-lime-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(236 252 203 / var(--tw-border-opacity))",
})
;"##### ; "97")]
#[test_case(r#####"tw`border-lime-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(217 249 157 / var(--tw-border-opacity))",
})
;"##### ; "98")]
#[test_case(r#####"tw`border-lime-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(190 242 100 / var(--tw-border-opacity))",
})
;"##### ; "99")]
#[test_case(r#####"tw`border-lime-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(163 230 53 / var(--tw-border-opacity))",
})
;"##### ; "100")]
#[test_case(r#####"tw`border-lime-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(132 204 22 / var(--tw-border-opacity))",
})
;"##### ; "101")]
#[test_case(r#####"tw`border-lime-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(101 163 13 / var(--tw-border-opacity))",
})
;"##### ; "102")]
#[test_case(r#####"tw`border-lime-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(77 124 15 / var(--tw-border-opacity))",
})
;"##### ; "103")]
#[test_case(r#####"tw`border-lime-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(63 98 18 / var(--tw-border-opacity))",
})
;"##### ; "104")]
#[test_case(r#####"tw`border-lime-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(54 83 20 / var(--tw-border-opacity))",
})
;"##### ; "105")]
#[test_case(r#####"tw`border-green-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(240 253 244 / var(--tw-border-opacity))",
})
;"##### ; "106")]
#[test_case(r#####"tw`border-green-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(220 252 231 / var(--tw-border-opacity))",
})
;"##### ; "107")]
#[test_case(r#####"tw`border-green-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(187 247 208 / var(--tw-border-opacity))",
})
;"##### ; "108")]
#[test_case(r#####"tw`border-green-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(134 239 172 / var(--tw-border-opacity))",
})
;"##### ; "109")]
#[test_case(r#####"tw`border-green-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(74 222 128 / var(--tw-border-opacity))",
})
;"##### ; "110")]
#[test_case(r#####"tw`border-green-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(34 197 94 / var(--tw-border-opacity))",
})
;"##### ; "111")]
#[test_case(r#####"tw`border-green-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(22 163 74 / var(--tw-border-opacity))",
})
;"##### ; "112")]
#[test_case(r#####"tw`border-green-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(21 128 61 / var(--tw-border-opacity))",
})
;"##### ; "113")]
#[test_case(r#####"tw`border-green-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(22 101 52 / var(--tw-border-opacity))",
})
;"##### ; "114")]
#[test_case(r#####"tw`border-green-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(20 83 45 / var(--tw-border-opacity))",
})
;"##### ; "115")]
#[test_case(r#####"tw`border-emerald-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(236 253 245 / var(--tw-border-opacity))",
})
;"##### ; "116")]
#[test_case(r#####"tw`border-emerald-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(209 250 229 / var(--tw-border-opacity))",
})
;"##### ; "117")]
#[test_case(r#####"tw`border-emerald-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(167 243 208 / var(--tw-border-opacity))",
})
;"##### ; "118")]
#[test_case(r#####"tw`border-emerald-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(110 231 183 / var(--tw-border-opacity))",
})
;"##### ; "119")]
#[test_case(r#####"tw`border-emerald-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(52 211 153 / var(--tw-border-opacity))",
})
;"##### ; "120")]
#[test_case(r#####"tw`border-emerald-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(16 185 129 / var(--tw-border-opacity))",
})
;"##### ; "121")]
#[test_case(r#####"tw`border-emerald-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(5 150 105 / var(--tw-border-opacity))",
})
;"##### ; "122")]
#[test_case(r#####"tw`border-emerald-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(4 120 87 / var(--tw-border-opacity))",
})
;"##### ; "123")]
#[test_case(r#####"tw`border-emerald-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(6 95 70 / var(--tw-border-opacity))",
})
;"##### ; "124")]
#[test_case(r#####"tw`border-emerald-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(6 78 59 / var(--tw-border-opacity))",
})
;"##### ; "125")]
#[test_case(r#####"tw`border-teal-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(240 253 250 / var(--tw-border-opacity))",
})
;"##### ; "126")]
#[test_case(r#####"tw`border-teal-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(204 251 241 / var(--tw-border-opacity))",
})
;"##### ; "127")]
#[test_case(r#####"tw`border-teal-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(153 246 228 / var(--tw-border-opacity))",
})
;"##### ; "128")]
#[test_case(r#####"tw`border-teal-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(94 234 212 / var(--tw-border-opacity))",
})
;"##### ; "129")]
#[test_case(r#####"tw`border-teal-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(45 212 191 / var(--tw-border-opacity))",
})
;"##### ; "130")]
#[test_case(r#####"tw`border-teal-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(20 184 166 / var(--tw-border-opacity))",
})
;"##### ; "131")]
#[test_case(r#####"tw`border-teal-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(13 148 136 / var(--tw-border-opacity))",
})
;"##### ; "132")]
#[test_case(r#####"tw`border-teal-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(15 118 110 / var(--tw-border-opacity))",
})
;"##### ; "133")]
#[test_case(r#####"tw`border-teal-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(17 94 89 / var(--tw-border-opacity))",
})
;"##### ; "134")]
#[test_case(r#####"tw`border-teal-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(19 78 74 / var(--tw-border-opacity))",
})
;"##### ; "135")]
#[test_case(r#####"tw`border-cyan-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(236 254 255 / var(--tw-border-opacity))",
})
;"##### ; "136")]
#[test_case(r#####"tw`border-cyan-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(207 250 254 / var(--tw-border-opacity))",
})
;"##### ; "137")]
#[test_case(r#####"tw`border-cyan-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(165 243 252 / var(--tw-border-opacity))",
})
;"##### ; "138")]
#[test_case(r#####"tw`border-cyan-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(103 232 249 / var(--tw-border-opacity))",
})
;"##### ; "139")]
#[test_case(r#####"tw`border-cyan-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(34 211 238 / var(--tw-border-opacity))",
})
;"##### ; "140")]
#[test_case(r#####"tw`border-cyan-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(6 182 212 / var(--tw-border-opacity))",
})
;"##### ; "141")]
#[test_case(r#####"tw`border-cyan-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(8 145 178 / var(--tw-border-opacity))",
})
;"##### ; "142")]
#[test_case(r#####"tw`border-cyan-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(14 116 144 / var(--tw-border-opacity))",
})
;"##### ; "143")]
#[test_case(r#####"tw`border-cyan-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(21 94 117 / var(--tw-border-opacity))",
})
;"##### ; "144")]
#[test_case(r#####"tw`border-cyan-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(22 78 99 / var(--tw-border-opacity))",
})
;"##### ; "145")]
#[test_case(r#####"tw`border-sky-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(240 249 255 / var(--tw-border-opacity))",
})
;"##### ; "146")]
#[test_case(r#####"tw`border-sky-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(224 242 254 / var(--tw-border-opacity))",
})
;"##### ; "147")]
#[test_case(r#####"tw`border-sky-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(186 230 253 / var(--tw-border-opacity))",
})
;"##### ; "148")]
#[test_case(r#####"tw`border-sky-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(125 211 252 / var(--tw-border-opacity))",
})
;"##### ; "149")]
#[test_case(r#####"tw`border-sky-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(56 189 248 / var(--tw-border-opacity))",
})
;"##### ; "150")]
#[test_case(r#####"tw`border-sky-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(14 165 233 / var(--tw-border-opacity))",
})
;"##### ; "151")]
#[test_case(r#####"tw`border-sky-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(2 132 199 / var(--tw-border-opacity))",
})
;"##### ; "152")]
#[test_case(r#####"tw`border-sky-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(3 105 161 / var(--tw-border-opacity))",
})
;"##### ; "153")]
#[test_case(r#####"tw`border-sky-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(7 89 133 / var(--tw-border-opacity))",
})
;"##### ; "154")]
#[test_case(r#####"tw`border-sky-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(12 74 110 / var(--tw-border-opacity))",
})
;"##### ; "155")]
#[test_case(r#####"tw`border-blue-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(239 246 255 / var(--tw-border-opacity))",
})
;"##### ; "156")]
#[test_case(r#####"tw`border-blue-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(219 234 254 / var(--tw-border-opacity))",
})
;"##### ; "157")]
#[test_case(r#####"tw`border-blue-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(191 219 254 / var(--tw-border-opacity))",
})
;"##### ; "158")]
#[test_case(r#####"tw`border-blue-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(147 197 253 / var(--tw-border-opacity))",
})
;"##### ; "159")]
#[test_case(r#####"tw`border-blue-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(96 165 250 / var(--tw-border-opacity))",
})
;"##### ; "160")]
#[test_case(r#####"tw`border-blue-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(59 130 246 / var(--tw-border-opacity))",
})
;"##### ; "161")]
#[test_case(r#####"tw`border-blue-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(37 99 235 / var(--tw-border-opacity))",
})
;"##### ; "162")]
#[test_case(r#####"tw`border-blue-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(29 78 216 / var(--tw-border-opacity))",
})
;"##### ; "163")]
#[test_case(r#####"tw`border-blue-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(30 64 175 / var(--tw-border-opacity))",
})
;"##### ; "164")]
#[test_case(r#####"tw`border-blue-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(30 58 138 / var(--tw-border-opacity))",
})
;"##### ; "165")]
#[test_case(r#####"tw`border-indigo-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(238 242 255 / var(--tw-border-opacity))",
})
;"##### ; "166")]
#[test_case(r#####"tw`border-indigo-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(224 231 255 / var(--tw-border-opacity))",
})
;"##### ; "167")]
#[test_case(r#####"tw`border-indigo-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(199 210 254 / var(--tw-border-opacity))",
})
;"##### ; "168")]
#[test_case(r#####"tw`border-indigo-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(165 180 252 / var(--tw-border-opacity))",
})
;"##### ; "169")]
#[test_case(r#####"tw`border-indigo-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(129 140 248 / var(--tw-border-opacity))",
})
;"##### ; "170")]
#[test_case(r#####"tw`border-indigo-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(99 102 241 / var(--tw-border-opacity))",
})
;"##### ; "171")]
#[test_case(r#####"tw`border-indigo-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(79 70 229 / var(--tw-border-opacity))",
})
;"##### ; "172")]
#[test_case(r#####"tw`border-indigo-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(67 56 202 / var(--tw-border-opacity))",
})
;"##### ; "173")]
#[test_case(r#####"tw`border-indigo-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(55 48 163 / var(--tw-border-opacity))",
})
;"##### ; "174")]
#[test_case(r#####"tw`border-indigo-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(49 46 129 / var(--tw-border-opacity))",
})
;"##### ; "175")]
#[test_case(r#####"tw`border-violet-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(245 243 255 / var(--tw-border-opacity))",
})
;"##### ; "176")]
#[test_case(r#####"tw`border-violet-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(237 233 254 / var(--tw-border-opacity))",
})
;"##### ; "177")]
#[test_case(r#####"tw`border-violet-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(221 214 254 / var(--tw-border-opacity))",
})
;"##### ; "178")]
#[test_case(r#####"tw`border-violet-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(196 181 253 / var(--tw-border-opacity))",
})
;"##### ; "179")]
#[test_case(r#####"tw`border-violet-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(167 139 250 / var(--tw-border-opacity))",
})
;"##### ; "180")]
#[test_case(r#####"tw`border-violet-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(139 92 246 / var(--tw-border-opacity))",
})
;"##### ; "181")]
#[test_case(r#####"tw`border-violet-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(124 58 237 / var(--tw-border-opacity))",
})
;"##### ; "182")]
#[test_case(r#####"tw`border-violet-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(109 40 217 / var(--tw-border-opacity))",
})
;"##### ; "183")]
#[test_case(r#####"tw`border-violet-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(91 33 182 / var(--tw-border-opacity))",
})
;"##### ; "184")]
#[test_case(r#####"tw`border-violet-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(76 29 149 / var(--tw-border-opacity))",
})
;"##### ; "185")]
#[test_case(r#####"tw`border-purple-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(250 245 255 / var(--tw-border-opacity))",
})
;"##### ; "186")]
#[test_case(r#####"tw`border-purple-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(243 232 255 / var(--tw-border-opacity))",
})
;"##### ; "187")]
#[test_case(r#####"tw`border-purple-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(233 213 255 / var(--tw-border-opacity))",
})
;"##### ; "188")]
#[test_case(r#####"tw`border-purple-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(216 180 254 / var(--tw-border-opacity))",
})
;"##### ; "189")]
#[test_case(r#####"tw`border-purple-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(192 132 252 / var(--tw-border-opacity))",
})
;"##### ; "190")]
#[test_case(r#####"tw`border-purple-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(168 85 247 / var(--tw-border-opacity))",
})
;"##### ; "191")]
#[test_case(r#####"tw`border-purple-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(147 51 234 / var(--tw-border-opacity))",
})
;"##### ; "192")]
#[test_case(r#####"tw`border-purple-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(126 34 206 / var(--tw-border-opacity))",
})
;"##### ; "193")]
#[test_case(r#####"tw`border-purple-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(107 33 168 / var(--tw-border-opacity))",
})
;"##### ; "194")]
#[test_case(r#####"tw`border-purple-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(88 28 135 / var(--tw-border-opacity))",
})
;"##### ; "195")]
#[test_case(r#####"tw`border-fuchsia-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(253 244 255 / var(--tw-border-opacity))",
})
;"##### ; "196")]
#[test_case(r#####"tw`border-fuchsia-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(250 232 255 / var(--tw-border-opacity))",
})
;"##### ; "197")]
#[test_case(r#####"tw`border-fuchsia-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(245 208 254 / var(--tw-border-opacity))",
})
;"##### ; "198")]
#[test_case(r#####"tw`border-fuchsia-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(240 171 252 / var(--tw-border-opacity))",
})
;"##### ; "199")]
#[test_case(r#####"tw`border-fuchsia-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(232 121 249 / var(--tw-border-opacity))",
})
;"##### ; "200")]
#[test_case(r#####"tw`border-fuchsia-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(217 70 239 / var(--tw-border-opacity))",
})
;"##### ; "201")]
#[test_case(r#####"tw`border-fuchsia-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(192 38 211 / var(--tw-border-opacity))",
})
;"##### ; "202")]
#[test_case(r#####"tw`border-fuchsia-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(162 28 175 / var(--tw-border-opacity))",
})
;"##### ; "203")]
#[test_case(r#####"tw`border-fuchsia-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(134 25 143 / var(--tw-border-opacity))",
})
;"##### ; "204")]
#[test_case(r#####"tw`border-fuchsia-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(112 26 117 / var(--tw-border-opacity))",
})
;"##### ; "205")]
#[test_case(r#####"tw`border-pink-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(253 242 248 / var(--tw-border-opacity))",
})
;"##### ; "206")]
#[test_case(r#####"tw`border-pink-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(252 231 243 / var(--tw-border-opacity))",
})
;"##### ; "207")]
#[test_case(r#####"tw`border-pink-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(251 207 232 / var(--tw-border-opacity))",
})
;"##### ; "208")]
#[test_case(r#####"tw`border-pink-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(249 168 212 / var(--tw-border-opacity))",
})
;"##### ; "209")]
#[test_case(r#####"tw`border-pink-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(244 114 182 / var(--tw-border-opacity))",
})
;"##### ; "210")]
#[test_case(r#####"tw`border-pink-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(236 72 153 / var(--tw-border-opacity))",
})
;"##### ; "211")]
#[test_case(r#####"tw`border-pink-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(219 39 119 / var(--tw-border-opacity))",
})
;"##### ; "212")]
#[test_case(r#####"tw`border-pink-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(190 24 93 / var(--tw-border-opacity))",
})
;"##### ; "213")]
#[test_case(r#####"tw`border-pink-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(157 23 77 / var(--tw-border-opacity))",
})
;"##### ; "214")]
#[test_case(r#####"tw`border-pink-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(131 24 67 / var(--tw-border-opacity))",
})
;"##### ; "215")]
#[test_case(r#####"tw`border-rose-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(255 241 242 / var(--tw-border-opacity))",
})
;"##### ; "216")]
#[test_case(r#####"tw`border-rose-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(255 228 230 / var(--tw-border-opacity))",
})
;"##### ; "217")]
#[test_case(r#####"tw`border-rose-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(254 205 211 / var(--tw-border-opacity))",
})
;"##### ; "218")]
#[test_case(r#####"tw`border-rose-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(253 164 175 / var(--tw-border-opacity))",
})
;"##### ; "219")]
#[test_case(r#####"tw`border-rose-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(251 113 133 / var(--tw-border-opacity))",
})
;"##### ; "220")]
#[test_case(r#####"tw`border-rose-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(244 63 94 / var(--tw-border-opacity))",
})
;"##### ; "221")]
#[test_case(r#####"tw`border-rose-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(225 29 72 / var(--tw-border-opacity))",
})
;"##### ; "222")]
#[test_case(r#####"tw`border-rose-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(190 18 60 / var(--tw-border-opacity))",
})
;"##### ; "223")]
#[test_case(r#####"tw`border-rose-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(159 18 57 / var(--tw-border-opacity))",
})
;"##### ; "224")]
#[test_case(r#####"tw`border-rose-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(136 19 55 / var(--tw-border-opacity))",
})
;"##### ; "225")]
#[test_case(r#####"tw`border-x-inherit`"#####, r#####"({
  borderLeftColor: "inherit",
  borderRightColor: "inherit",
})
;"##### ; "226")]
#[test_case(r#####"tw`border-x-current`"#####, r#####"({
  borderLeftColor: "currentColor",
  borderRightColor: "currentColor",
})
;"##### ; "227")]
#[test_case(r#####"tw`border-x-transparent`"#####, r#####"({
  borderLeftColor: "transparent",
  borderRightColor: "transparent",
})
;"##### ; "228")]
#[test_case(r#####"tw`border-x-black`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(0 0 0 / var(--tw-border-opacity))",
  borderRightColor: "rgb(0 0 0 / var(--tw-border-opacity))",
})
;"##### ; "229")]
#[test_case(r#####"tw`border-x-white`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(255 255 255 / var(--tw-border-opacity))",
  borderRightColor: "rgb(255 255 255 / var(--tw-border-opacity))",
})
;"##### ; "230")]
#[test_case(r#####"tw`border-x-slate-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(248 250 252 / var(--tw-border-opacity))",
  borderRightColor: "rgb(248 250 252 / var(--tw-border-opacity))",
})
;"##### ; "231")]
#[test_case(r#####"tw`border-x-slate-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(241 245 249 / var(--tw-border-opacity))",
  borderRightColor: "rgb(241 245 249 / var(--tw-border-opacity))",
})
;"##### ; "232")]
#[test_case(r#####"tw`border-x-slate-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(226 232 240 / var(--tw-border-opacity))",
  borderRightColor: "rgb(226 232 240 / var(--tw-border-opacity))",
})
;"##### ; "233")]
#[test_case(r#####"tw`border-x-slate-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(203 213 225 / var(--tw-border-opacity))",
  borderRightColor: "rgb(203 213 225 / var(--tw-border-opacity))",
})
;"##### ; "234")]
#[test_case(r#####"tw`border-x-slate-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(148 163 184 / var(--tw-border-opacity))",
  borderRightColor: "rgb(148 163 184 / var(--tw-border-opacity))",
})
;"##### ; "235")]
#[test_case(r#####"tw`border-x-slate-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(100 116 139 / var(--tw-border-opacity))",
  borderRightColor: "rgb(100 116 139 / var(--tw-border-opacity))",
})
;"##### ; "236")]
#[test_case(r#####"tw`border-x-slate-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(71 85 105 / var(--tw-border-opacity))",
  borderRightColor: "rgb(71 85 105 / var(--tw-border-opacity))",
})
;"##### ; "237")]
#[test_case(r#####"tw`border-x-slate-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(51 65 85 / var(--tw-border-opacity))",
  borderRightColor: "rgb(51 65 85 / var(--tw-border-opacity))",
})
;"##### ; "238")]
#[test_case(r#####"tw`border-x-slate-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(30 41 59 / var(--tw-border-opacity))",
  borderRightColor: "rgb(30 41 59 / var(--tw-border-opacity))",
})
;"##### ; "239")]
#[test_case(r#####"tw`border-x-slate-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(15 23 42 / var(--tw-border-opacity))",
  borderRightColor: "rgb(15 23 42 / var(--tw-border-opacity))",
})
;"##### ; "240")]
#[test_case(r#####"tw`border-x-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(249 250 251 / var(--tw-border-opacity))",
  borderRightColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "241")]
#[test_case(r#####"tw`border-x-gray-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(243 244 246 / var(--tw-border-opacity))",
  borderRightColor: "rgb(243 244 246 / var(--tw-border-opacity))",
})
;"##### ; "242")]
#[test_case(r#####"tw`border-x-gray-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(229 231 235 / var(--tw-border-opacity))",
  borderRightColor: "rgb(229 231 235 / var(--tw-border-opacity))",
})
;"##### ; "243")]
#[test_case(r#####"tw`border-x-gray-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(209 213 219 / var(--tw-border-opacity))",
  borderRightColor: "rgb(209 213 219 / var(--tw-border-opacity))",
})
;"##### ; "244")]
#[test_case(r#####"tw`border-x-gray-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(156 163 175 / var(--tw-border-opacity))",
  borderRightColor: "rgb(156 163 175 / var(--tw-border-opacity))",
})
;"##### ; "245")]
#[test_case(r#####"tw`border-x-gray-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(107 114 128 / var(--tw-border-opacity))",
  borderRightColor: "rgb(107 114 128 / var(--tw-border-opacity))",
})
;"##### ; "246")]
#[test_case(r#####"tw`border-x-gray-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(75 85 99 / var(--tw-border-opacity))",
  borderRightColor: "rgb(75 85 99 / var(--tw-border-opacity))",
})
;"##### ; "247")]
#[test_case(r#####"tw`border-x-gray-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(55 65 81 / var(--tw-border-opacity))",
  borderRightColor: "rgb(55 65 81 / var(--tw-border-opacity))",
})
;"##### ; "248")]
#[test_case(r#####"tw`border-x-gray-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(31 41 55 / var(--tw-border-opacity))",
  borderRightColor: "rgb(31 41 55 / var(--tw-border-opacity))",
})
;"##### ; "249")]
#[test_case(r#####"tw`border-x-gray-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(17 24 39 / var(--tw-border-opacity))",
  borderRightColor: "rgb(17 24 39 / var(--tw-border-opacity))",
})
;"##### ; "250")]
#[test_case(r#####"tw`border-x-zinc-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(250 250 250 / var(--tw-border-opacity))",
  borderRightColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "251")]
#[test_case(r#####"tw`border-x-zinc-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(244 244 245 / var(--tw-border-opacity))",
  borderRightColor: "rgb(244 244 245 / var(--tw-border-opacity))",
})
;"##### ; "252")]
#[test_case(r#####"tw`border-x-zinc-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(228 228 231 / var(--tw-border-opacity))",
  borderRightColor: "rgb(228 228 231 / var(--tw-border-opacity))",
})
;"##### ; "253")]
#[test_case(r#####"tw`border-x-zinc-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(212 212 216 / var(--tw-border-opacity))",
  borderRightColor: "rgb(212 212 216 / var(--tw-border-opacity))",
})
;"##### ; "254")]
#[test_case(r#####"tw`border-x-zinc-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(161 161 170 / var(--tw-border-opacity))",
  borderRightColor: "rgb(161 161 170 / var(--tw-border-opacity))",
})
;"##### ; "255")]
#[test_case(r#####"tw`border-x-zinc-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(113 113 122 / var(--tw-border-opacity))",
  borderRightColor: "rgb(113 113 122 / var(--tw-border-opacity))",
})
;"##### ; "256")]
#[test_case(r#####"tw`border-x-zinc-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(82 82 91 / var(--tw-border-opacity))",
  borderRightColor: "rgb(82 82 91 / var(--tw-border-opacity))",
})
;"##### ; "257")]
#[test_case(r#####"tw`border-x-zinc-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(63 63 70 / var(--tw-border-opacity))",
  borderRightColor: "rgb(63 63 70 / var(--tw-border-opacity))",
})
;"##### ; "258")]
#[test_case(r#####"tw`border-x-zinc-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(39 39 42 / var(--tw-border-opacity))",
  borderRightColor: "rgb(39 39 42 / var(--tw-border-opacity))",
})
;"##### ; "259")]
#[test_case(r#####"tw`border-x-zinc-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(24 24 27 / var(--tw-border-opacity))",
  borderRightColor: "rgb(24 24 27 / var(--tw-border-opacity))",
})
;"##### ; "260")]
#[test_case(r#####"tw`border-x-neutral-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(250 250 250 / var(--tw-border-opacity))",
  borderRightColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "261")]
#[test_case(r#####"tw`border-x-neutral-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(245 245 245 / var(--tw-border-opacity))",
  borderRightColor: "rgb(245 245 245 / var(--tw-border-opacity))",
})
;"##### ; "262")]
#[test_case(r#####"tw`border-x-neutral-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(229 229 229 / var(--tw-border-opacity))",
  borderRightColor: "rgb(229 229 229 / var(--tw-border-opacity))",
})
;"##### ; "263")]
#[test_case(r#####"tw`border-x-neutral-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(212 212 212 / var(--tw-border-opacity))",
  borderRightColor: "rgb(212 212 212 / var(--tw-border-opacity))",
})
;"##### ; "264")]
#[test_case(r#####"tw`border-x-neutral-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(163 163 163 / var(--tw-border-opacity))",
  borderRightColor: "rgb(163 163 163 / var(--tw-border-opacity))",
})
;"##### ; "265")]
#[test_case(r#####"tw`border-x-neutral-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(115 115 115 / var(--tw-border-opacity))",
  borderRightColor: "rgb(115 115 115 / var(--tw-border-opacity))",
})
;"##### ; "266")]
#[test_case(r#####"tw`border-x-neutral-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(82 82 82 / var(--tw-border-opacity))",
  borderRightColor: "rgb(82 82 82 / var(--tw-border-opacity))",
})
;"##### ; "267")]
#[test_case(r#####"tw`border-x-neutral-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(64 64 64 / var(--tw-border-opacity))",
  borderRightColor: "rgb(64 64 64 / var(--tw-border-opacity))",
})
;"##### ; "268")]
#[test_case(r#####"tw`border-x-neutral-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(38 38 38 / var(--tw-border-opacity))",
  borderRightColor: "rgb(38 38 38 / var(--tw-border-opacity))",
})
;"##### ; "269")]
#[test_case(r#####"tw`border-x-neutral-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(23 23 23 / var(--tw-border-opacity))",
  borderRightColor: "rgb(23 23 23 / var(--tw-border-opacity))",
})
;"##### ; "270")]
#[test_case(r#####"tw`border-x-stone-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(250 250 249 / var(--tw-border-opacity))",
  borderRightColor: "rgb(250 250 249 / var(--tw-border-opacity))",
})
;"##### ; "271")]
#[test_case(r#####"tw`border-x-stone-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(245 245 244 / var(--tw-border-opacity))",
  borderRightColor: "rgb(245 245 244 / var(--tw-border-opacity))",
})
;"##### ; "272")]
#[test_case(r#####"tw`border-x-stone-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(231 229 228 / var(--tw-border-opacity))",
  borderRightColor: "rgb(231 229 228 / var(--tw-border-opacity))",
})
;"##### ; "273")]
#[test_case(r#####"tw`border-x-stone-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(214 211 209 / var(--tw-border-opacity))",
  borderRightColor: "rgb(214 211 209 / var(--tw-border-opacity))",
})
;"##### ; "274")]
#[test_case(r#####"tw`border-x-stone-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(168 162 158 / var(--tw-border-opacity))",
  borderRightColor: "rgb(168 162 158 / var(--tw-border-opacity))",
})
;"##### ; "275")]
#[test_case(r#####"tw`border-x-stone-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(120 113 108 / var(--tw-border-opacity))",
  borderRightColor: "rgb(120 113 108 / var(--tw-border-opacity))",
})
;"##### ; "276")]
#[test_case(r#####"tw`border-x-stone-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(87 83 78 / var(--tw-border-opacity))",
  borderRightColor: "rgb(87 83 78 / var(--tw-border-opacity))",
})
;"##### ; "277")]
#[test_case(r#####"tw`border-x-stone-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(68 64 60 / var(--tw-border-opacity))",
  borderRightColor: "rgb(68 64 60 / var(--tw-border-opacity))",
})
;"##### ; "278")]
#[test_case(r#####"tw`border-x-stone-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(41 37 36 / var(--tw-border-opacity))",
  borderRightColor: "rgb(41 37 36 / var(--tw-border-opacity))",
})
;"##### ; "279")]
#[test_case(r#####"tw`border-x-stone-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(28 25 23 / var(--tw-border-opacity))",
  borderRightColor: "rgb(28 25 23 / var(--tw-border-opacity))",
})
;"##### ; "280")]
#[test_case(r#####"tw`border-x-red-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 242 242 / var(--tw-border-opacity))",
  borderRightColor: "rgb(254 242 242 / var(--tw-border-opacity))",
})
;"##### ; "281")]
#[test_case(r#####"tw`border-x-red-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 226 226 / var(--tw-border-opacity))",
  borderRightColor: "rgb(254 226 226 / var(--tw-border-opacity))",
})
;"##### ; "282")]
#[test_case(r#####"tw`border-x-red-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 202 202 / var(--tw-border-opacity))",
  borderRightColor: "rgb(254 202 202 / var(--tw-border-opacity))",
})
;"##### ; "283")]
#[test_case(r#####"tw`border-x-red-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(252 165 165 / var(--tw-border-opacity))",
  borderRightColor: "rgb(252 165 165 / var(--tw-border-opacity))",
})
;"##### ; "284")]
#[test_case(r#####"tw`border-x-red-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(248 113 113 / var(--tw-border-opacity))",
  borderRightColor: "rgb(248 113 113 / var(--tw-border-opacity))",
})
;"##### ; "285")]
#[test_case(r#####"tw`border-x-red-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(239 68 68 / var(--tw-border-opacity))",
  borderRightColor: "rgb(239 68 68 / var(--tw-border-opacity))",
})
;"##### ; "286")]
#[test_case(r#####"tw`border-x-red-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(220 38 38 / var(--tw-border-opacity))",
  borderRightColor: "rgb(220 38 38 / var(--tw-border-opacity))",
})
;"##### ; "287")]
#[test_case(r#####"tw`border-x-red-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(185 28 28 / var(--tw-border-opacity))",
  borderRightColor: "rgb(185 28 28 / var(--tw-border-opacity))",
})
;"##### ; "288")]
#[test_case(r#####"tw`border-x-red-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(153 27 27 / var(--tw-border-opacity))",
  borderRightColor: "rgb(153 27 27 / var(--tw-border-opacity))",
})
;"##### ; "289")]
#[test_case(r#####"tw`border-x-red-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(127 29 29 / var(--tw-border-opacity))",
  borderRightColor: "rgb(127 29 29 / var(--tw-border-opacity))",
})
;"##### ; "290")]
#[test_case(r#####"tw`border-x-orange-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(255 247 237 / var(--tw-border-opacity))",
  borderRightColor: "rgb(255 247 237 / var(--tw-border-opacity))",
})
;"##### ; "291")]
#[test_case(r#####"tw`border-x-orange-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(255 237 213 / var(--tw-border-opacity))",
  borderRightColor: "rgb(255 237 213 / var(--tw-border-opacity))",
})
;"##### ; "292")]
#[test_case(r#####"tw`border-x-orange-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 215 170 / var(--tw-border-opacity))",
  borderRightColor: "rgb(254 215 170 / var(--tw-border-opacity))",
})
;"##### ; "293")]
#[test_case(r#####"tw`border-x-orange-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(253 186 116 / var(--tw-border-opacity))",
  borderRightColor: "rgb(253 186 116 / var(--tw-border-opacity))",
})
;"##### ; "294")]
#[test_case(r#####"tw`border-x-orange-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(251 146 60 / var(--tw-border-opacity))",
  borderRightColor: "rgb(251 146 60 / var(--tw-border-opacity))",
})
;"##### ; "295")]
#[test_case(r#####"tw`border-x-orange-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(249 115 22 / var(--tw-border-opacity))",
  borderRightColor: "rgb(249 115 22 / var(--tw-border-opacity))",
})
;"##### ; "296")]
#[test_case(r#####"tw`border-x-orange-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(234 88 12 / var(--tw-border-opacity))",
  borderRightColor: "rgb(234 88 12 / var(--tw-border-opacity))",
})
;"##### ; "297")]
#[test_case(r#####"tw`border-x-orange-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(194 65 12 / var(--tw-border-opacity))",
  borderRightColor: "rgb(194 65 12 / var(--tw-border-opacity))",
})
;"##### ; "298")]
#[test_case(r#####"tw`border-x-orange-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(154 52 18 / var(--tw-border-opacity))",
  borderRightColor: "rgb(154 52 18 / var(--tw-border-opacity))",
})
;"##### ; "299")]
#[test_case(r#####"tw`border-x-orange-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(124 45 18 / var(--tw-border-opacity))",
  borderRightColor: "rgb(124 45 18 / var(--tw-border-opacity))",
})
;"##### ; "300")]
#[test_case(r#####"tw`border-x-amber-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(255 251 235 / var(--tw-border-opacity))",
  borderRightColor: "rgb(255 251 235 / var(--tw-border-opacity))",
})
;"##### ; "301")]
#[test_case(r#####"tw`border-x-amber-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 243 199 / var(--tw-border-opacity))",
  borderRightColor: "rgb(254 243 199 / var(--tw-border-opacity))",
})
;"##### ; "302")]
#[test_case(r#####"tw`border-x-amber-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(253 230 138 / var(--tw-border-opacity))",
  borderRightColor: "rgb(253 230 138 / var(--tw-border-opacity))",
})
;"##### ; "303")]
#[test_case(r#####"tw`border-x-amber-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(252 211 77 / var(--tw-border-opacity))",
  borderRightColor: "rgb(252 211 77 / var(--tw-border-opacity))",
})
;"##### ; "304")]
#[test_case(r#####"tw`border-x-amber-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(251 191 36 / var(--tw-border-opacity))",
  borderRightColor: "rgb(251 191 36 / var(--tw-border-opacity))",
})
;"##### ; "305")]
#[test_case(r#####"tw`border-x-amber-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(245 158 11 / var(--tw-border-opacity))",
  borderRightColor: "rgb(245 158 11 / var(--tw-border-opacity))",
})
;"##### ; "306")]
#[test_case(r#####"tw`border-x-amber-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(217 119 6 / var(--tw-border-opacity))",
  borderRightColor: "rgb(217 119 6 / var(--tw-border-opacity))",
})
;"##### ; "307")]
#[test_case(r#####"tw`border-x-amber-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(180 83 9 / var(--tw-border-opacity))",
  borderRightColor: "rgb(180 83 9 / var(--tw-border-opacity))",
})
;"##### ; "308")]
#[test_case(r#####"tw`border-x-amber-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(146 64 14 / var(--tw-border-opacity))",
  borderRightColor: "rgb(146 64 14 / var(--tw-border-opacity))",
})
;"##### ; "309")]
#[test_case(r#####"tw`border-x-amber-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(120 53 15 / var(--tw-border-opacity))",
  borderRightColor: "rgb(120 53 15 / var(--tw-border-opacity))",
})
;"##### ; "310")]
#[test_case(r#####"tw`border-x-yellow-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 252 232 / var(--tw-border-opacity))",
  borderRightColor: "rgb(254 252 232 / var(--tw-border-opacity))",
})
;"##### ; "311")]
#[test_case(r#####"tw`border-x-yellow-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 249 195 / var(--tw-border-opacity))",
  borderRightColor: "rgb(254 249 195 / var(--tw-border-opacity))",
})
;"##### ; "312")]
#[test_case(r#####"tw`border-x-yellow-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 240 138 / var(--tw-border-opacity))",
  borderRightColor: "rgb(254 240 138 / var(--tw-border-opacity))",
})
;"##### ; "313")]
#[test_case(r#####"tw`border-x-yellow-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(253 224 71 / var(--tw-border-opacity))",
  borderRightColor: "rgb(253 224 71 / var(--tw-border-opacity))",
})
;"##### ; "314")]
#[test_case(r#####"tw`border-x-yellow-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(250 204 21 / var(--tw-border-opacity))",
  borderRightColor: "rgb(250 204 21 / var(--tw-border-opacity))",
})
;"##### ; "315")]
#[test_case(r#####"tw`border-x-yellow-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(234 179 8 / var(--tw-border-opacity))",
  borderRightColor: "rgb(234 179 8 / var(--tw-border-opacity))",
})
;"##### ; "316")]
#[test_case(r#####"tw`border-x-yellow-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(202 138 4 / var(--tw-border-opacity))",
  borderRightColor: "rgb(202 138 4 / var(--tw-border-opacity))",
})
;"##### ; "317")]
#[test_case(r#####"tw`border-x-yellow-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(161 98 7 / var(--tw-border-opacity))",
  borderRightColor: "rgb(161 98 7 / var(--tw-border-opacity))",
})
;"##### ; "318")]
#[test_case(r#####"tw`border-x-yellow-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(133 77 14 / var(--tw-border-opacity))",
  borderRightColor: "rgb(133 77 14 / var(--tw-border-opacity))",
})
;"##### ; "319")]
#[test_case(r#####"tw`border-x-yellow-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(113 63 18 / var(--tw-border-opacity))",
  borderRightColor: "rgb(113 63 18 / var(--tw-border-opacity))",
})
;"##### ; "320")]
#[test_case(r#####"tw`border-x-lime-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(247 254 231 / var(--tw-border-opacity))",
  borderRightColor: "rgb(247 254 231 / var(--tw-border-opacity))",
})
;"##### ; "321")]
#[test_case(r#####"tw`border-x-lime-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(236 252 203 / var(--tw-border-opacity))",
  borderRightColor: "rgb(236 252 203 / var(--tw-border-opacity))",
})
;"##### ; "322")]
#[test_case(r#####"tw`border-x-lime-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(217 249 157 / var(--tw-border-opacity))",
  borderRightColor: "rgb(217 249 157 / var(--tw-border-opacity))",
})
;"##### ; "323")]
#[test_case(r#####"tw`border-x-lime-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(190 242 100 / var(--tw-border-opacity))",
  borderRightColor: "rgb(190 242 100 / var(--tw-border-opacity))",
})
;"##### ; "324")]
#[test_case(r#####"tw`border-x-lime-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(163 230 53 / var(--tw-border-opacity))",
  borderRightColor: "rgb(163 230 53 / var(--tw-border-opacity))",
})
;"##### ; "325")]
#[test_case(r#####"tw`border-x-lime-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(132 204 22 / var(--tw-border-opacity))",
  borderRightColor: "rgb(132 204 22 / var(--tw-border-opacity))",
})
;"##### ; "326")]
#[test_case(r#####"tw`border-x-lime-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(101 163 13 / var(--tw-border-opacity))",
  borderRightColor: "rgb(101 163 13 / var(--tw-border-opacity))",
})
;"##### ; "327")]
#[test_case(r#####"tw`border-x-lime-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(77 124 15 / var(--tw-border-opacity))",
  borderRightColor: "rgb(77 124 15 / var(--tw-border-opacity))",
})
;"##### ; "328")]
#[test_case(r#####"tw`border-x-lime-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(63 98 18 / var(--tw-border-opacity))",
  borderRightColor: "rgb(63 98 18 / var(--tw-border-opacity))",
})
;"##### ; "329")]
#[test_case(r#####"tw`border-x-lime-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(54 83 20 / var(--tw-border-opacity))",
  borderRightColor: "rgb(54 83 20 / var(--tw-border-opacity))",
})
;"##### ; "330")]
#[test_case(r#####"tw`border-x-green-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(240 253 244 / var(--tw-border-opacity))",
  borderRightColor: "rgb(240 253 244 / var(--tw-border-opacity))",
})
;"##### ; "331")]
#[test_case(r#####"tw`border-x-green-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(220 252 231 / var(--tw-border-opacity))",
  borderRightColor: "rgb(220 252 231 / var(--tw-border-opacity))",
})
;"##### ; "332")]
#[test_case(r#####"tw`border-x-green-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(187 247 208 / var(--tw-border-opacity))",
  borderRightColor: "rgb(187 247 208 / var(--tw-border-opacity))",
})
;"##### ; "333")]
#[test_case(r#####"tw`border-x-green-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(134 239 172 / var(--tw-border-opacity))",
  borderRightColor: "rgb(134 239 172 / var(--tw-border-opacity))",
})
;"##### ; "334")]
#[test_case(r#####"tw`border-x-green-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(74 222 128 / var(--tw-border-opacity))",
  borderRightColor: "rgb(74 222 128 / var(--tw-border-opacity))",
})
;"##### ; "335")]
#[test_case(r#####"tw`border-x-green-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(34 197 94 / var(--tw-border-opacity))",
  borderRightColor: "rgb(34 197 94 / var(--tw-border-opacity))",
})
;"##### ; "336")]
#[test_case(r#####"tw`border-x-green-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(22 163 74 / var(--tw-border-opacity))",
  borderRightColor: "rgb(22 163 74 / var(--tw-border-opacity))",
})
;"##### ; "337")]
#[test_case(r#####"tw`border-x-green-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(21 128 61 / var(--tw-border-opacity))",
  borderRightColor: "rgb(21 128 61 / var(--tw-border-opacity))",
})
;"##### ; "338")]
#[test_case(r#####"tw`border-x-green-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(22 101 52 / var(--tw-border-opacity))",
  borderRightColor: "rgb(22 101 52 / var(--tw-border-opacity))",
})
;"##### ; "339")]
#[test_case(r#####"tw`border-x-green-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(20 83 45 / var(--tw-border-opacity))",
  borderRightColor: "rgb(20 83 45 / var(--tw-border-opacity))",
})
;"##### ; "340")]
#[test_case(r#####"tw`border-x-emerald-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(236 253 245 / var(--tw-border-opacity))",
  borderRightColor: "rgb(236 253 245 / var(--tw-border-opacity))",
})
;"##### ; "341")]
#[test_case(r#####"tw`border-x-emerald-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(209 250 229 / var(--tw-border-opacity))",
  borderRightColor: "rgb(209 250 229 / var(--tw-border-opacity))",
})
;"##### ; "342")]
#[test_case(r#####"tw`border-x-emerald-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(167 243 208 / var(--tw-border-opacity))",
  borderRightColor: "rgb(167 243 208 / var(--tw-border-opacity))",
})
;"##### ; "343")]
#[test_case(r#####"tw`border-x-emerald-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(110 231 183 / var(--tw-border-opacity))",
  borderRightColor: "rgb(110 231 183 / var(--tw-border-opacity))",
})
;"##### ; "344")]
#[test_case(r#####"tw`border-x-emerald-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(52 211 153 / var(--tw-border-opacity))",
  borderRightColor: "rgb(52 211 153 / var(--tw-border-opacity))",
})
;"##### ; "345")]
#[test_case(r#####"tw`border-x-emerald-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(16 185 129 / var(--tw-border-opacity))",
  borderRightColor: "rgb(16 185 129 / var(--tw-border-opacity))",
})
;"##### ; "346")]
#[test_case(r#####"tw`border-x-emerald-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(5 150 105 / var(--tw-border-opacity))",
  borderRightColor: "rgb(5 150 105 / var(--tw-border-opacity))",
})
;"##### ; "347")]
#[test_case(r#####"tw`border-x-emerald-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(4 120 87 / var(--tw-border-opacity))",
  borderRightColor: "rgb(4 120 87 / var(--tw-border-opacity))",
})
;"##### ; "348")]
#[test_case(r#####"tw`border-x-emerald-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(6 95 70 / var(--tw-border-opacity))",
  borderRightColor: "rgb(6 95 70 / var(--tw-border-opacity))",
})
;"##### ; "349")]
#[test_case(r#####"tw`border-x-emerald-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(6 78 59 / var(--tw-border-opacity))",
  borderRightColor: "rgb(6 78 59 / var(--tw-border-opacity))",
})
;"##### ; "350")]
#[test_case(r#####"tw`border-x-teal-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(240 253 250 / var(--tw-border-opacity))",
  borderRightColor: "rgb(240 253 250 / var(--tw-border-opacity))",
})
;"##### ; "351")]
#[test_case(r#####"tw`border-x-teal-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(204 251 241 / var(--tw-border-opacity))",
  borderRightColor: "rgb(204 251 241 / var(--tw-border-opacity))",
})
;"##### ; "352")]
#[test_case(r#####"tw`border-x-teal-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(153 246 228 / var(--tw-border-opacity))",
  borderRightColor: "rgb(153 246 228 / var(--tw-border-opacity))",
})
;"##### ; "353")]
#[test_case(r#####"tw`border-x-teal-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(94 234 212 / var(--tw-border-opacity))",
  borderRightColor: "rgb(94 234 212 / var(--tw-border-opacity))",
})
;"##### ; "354")]
#[test_case(r#####"tw`border-x-teal-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(45 212 191 / var(--tw-border-opacity))",
  borderRightColor: "rgb(45 212 191 / var(--tw-border-opacity))",
})
;"##### ; "355")]
#[test_case(r#####"tw`border-x-teal-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(20 184 166 / var(--tw-border-opacity))",
  borderRightColor: "rgb(20 184 166 / var(--tw-border-opacity))",
})
;"##### ; "356")]
#[test_case(r#####"tw`border-x-teal-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(13 148 136 / var(--tw-border-opacity))",
  borderRightColor: "rgb(13 148 136 / var(--tw-border-opacity))",
})
;"##### ; "357")]
#[test_case(r#####"tw`border-x-teal-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(15 118 110 / var(--tw-border-opacity))",
  borderRightColor: "rgb(15 118 110 / var(--tw-border-opacity))",
})
;"##### ; "358")]
#[test_case(r#####"tw`border-x-teal-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(17 94 89 / var(--tw-border-opacity))",
  borderRightColor: "rgb(17 94 89 / var(--tw-border-opacity))",
})
;"##### ; "359")]
#[test_case(r#####"tw`border-x-teal-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(19 78 74 / var(--tw-border-opacity))",
  borderRightColor: "rgb(19 78 74 / var(--tw-border-opacity))",
})
;"##### ; "360")]
#[test_case(r#####"tw`border-x-cyan-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(236 254 255 / var(--tw-border-opacity))",
  borderRightColor: "rgb(236 254 255 / var(--tw-border-opacity))",
})
;"##### ; "361")]
#[test_case(r#####"tw`border-x-cyan-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(207 250 254 / var(--tw-border-opacity))",
  borderRightColor: "rgb(207 250 254 / var(--tw-border-opacity))",
})
;"##### ; "362")]
#[test_case(r#####"tw`border-x-cyan-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(165 243 252 / var(--tw-border-opacity))",
  borderRightColor: "rgb(165 243 252 / var(--tw-border-opacity))",
})
;"##### ; "363")]
#[test_case(r#####"tw`border-x-cyan-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(103 232 249 / var(--tw-border-opacity))",
  borderRightColor: "rgb(103 232 249 / var(--tw-border-opacity))",
})
;"##### ; "364")]
#[test_case(r#####"tw`border-x-cyan-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(34 211 238 / var(--tw-border-opacity))",
  borderRightColor: "rgb(34 211 238 / var(--tw-border-opacity))",
})
;"##### ; "365")]
#[test_case(r#####"tw`border-x-cyan-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(6 182 212 / var(--tw-border-opacity))",
  borderRightColor: "rgb(6 182 212 / var(--tw-border-opacity))",
})
;"##### ; "366")]
#[test_case(r#####"tw`border-x-cyan-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(8 145 178 / var(--tw-border-opacity))",
  borderRightColor: "rgb(8 145 178 / var(--tw-border-opacity))",
})
;"##### ; "367")]
#[test_case(r#####"tw`border-x-cyan-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(14 116 144 / var(--tw-border-opacity))",
  borderRightColor: "rgb(14 116 144 / var(--tw-border-opacity))",
})
;"##### ; "368")]
#[test_case(r#####"tw`border-x-cyan-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(21 94 117 / var(--tw-border-opacity))",
  borderRightColor: "rgb(21 94 117 / var(--tw-border-opacity))",
})
;"##### ; "369")]
#[test_case(r#####"tw`border-x-cyan-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(22 78 99 / var(--tw-border-opacity))",
  borderRightColor: "rgb(22 78 99 / var(--tw-border-opacity))",
})
;"##### ; "370")]
#[test_case(r#####"tw`border-x-sky-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(240 249 255 / var(--tw-border-opacity))",
  borderRightColor: "rgb(240 249 255 / var(--tw-border-opacity))",
})
;"##### ; "371")]
#[test_case(r#####"tw`border-x-sky-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(224 242 254 / var(--tw-border-opacity))",
  borderRightColor: "rgb(224 242 254 / var(--tw-border-opacity))",
})
;"##### ; "372")]
#[test_case(r#####"tw`border-x-sky-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(186 230 253 / var(--tw-border-opacity))",
  borderRightColor: "rgb(186 230 253 / var(--tw-border-opacity))",
})
;"##### ; "373")]
#[test_case(r#####"tw`border-x-sky-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(125 211 252 / var(--tw-border-opacity))",
  borderRightColor: "rgb(125 211 252 / var(--tw-border-opacity))",
})
;"##### ; "374")]
#[test_case(r#####"tw`border-x-sky-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(56 189 248 / var(--tw-border-opacity))",
  borderRightColor: "rgb(56 189 248 / var(--tw-border-opacity))",
})
;"##### ; "375")]
#[test_case(r#####"tw`border-x-sky-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(14 165 233 / var(--tw-border-opacity))",
  borderRightColor: "rgb(14 165 233 / var(--tw-border-opacity))",
})
;"##### ; "376")]
#[test_case(r#####"tw`border-x-sky-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(2 132 199 / var(--tw-border-opacity))",
  borderRightColor: "rgb(2 132 199 / var(--tw-border-opacity))",
})
;"##### ; "377")]
#[test_case(r#####"tw`border-x-sky-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(3 105 161 / var(--tw-border-opacity))",
  borderRightColor: "rgb(3 105 161 / var(--tw-border-opacity))",
})
;"##### ; "378")]
#[test_case(r#####"tw`border-x-sky-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(7 89 133 / var(--tw-border-opacity))",
  borderRightColor: "rgb(7 89 133 / var(--tw-border-opacity))",
})
;"##### ; "379")]
#[test_case(r#####"tw`border-x-sky-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(12 74 110 / var(--tw-border-opacity))",
  borderRightColor: "rgb(12 74 110 / var(--tw-border-opacity))",
})
;"##### ; "380")]
#[test_case(r#####"tw`border-x-blue-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(239 246 255 / var(--tw-border-opacity))",
  borderRightColor: "rgb(239 246 255 / var(--tw-border-opacity))",
})
;"##### ; "381")]
#[test_case(r#####"tw`border-x-blue-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(219 234 254 / var(--tw-border-opacity))",
  borderRightColor: "rgb(219 234 254 / var(--tw-border-opacity))",
})
;"##### ; "382")]
#[test_case(r#####"tw`border-x-blue-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(191 219 254 / var(--tw-border-opacity))",
  borderRightColor: "rgb(191 219 254 / var(--tw-border-opacity))",
})
;"##### ; "383")]
#[test_case(r#####"tw`border-x-blue-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(147 197 253 / var(--tw-border-opacity))",
  borderRightColor: "rgb(147 197 253 / var(--tw-border-opacity))",
})
;"##### ; "384")]
#[test_case(r#####"tw`border-x-blue-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(96 165 250 / var(--tw-border-opacity))",
  borderRightColor: "rgb(96 165 250 / var(--tw-border-opacity))",
})
;"##### ; "385")]
#[test_case(r#####"tw`border-x-blue-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(59 130 246 / var(--tw-border-opacity))",
  borderRightColor: "rgb(59 130 246 / var(--tw-border-opacity))",
})
;"##### ; "386")]
#[test_case(r#####"tw`border-x-blue-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(37 99 235 / var(--tw-border-opacity))",
  borderRightColor: "rgb(37 99 235 / var(--tw-border-opacity))",
})
;"##### ; "387")]
#[test_case(r#####"tw`border-x-blue-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(29 78 216 / var(--tw-border-opacity))",
  borderRightColor: "rgb(29 78 216 / var(--tw-border-opacity))",
})
;"##### ; "388")]
#[test_case(r#####"tw`border-x-blue-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(30 64 175 / var(--tw-border-opacity))",
  borderRightColor: "rgb(30 64 175 / var(--tw-border-opacity))",
})
;"##### ; "389")]
#[test_case(r#####"tw`border-x-blue-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(30 58 138 / var(--tw-border-opacity))",
  borderRightColor: "rgb(30 58 138 / var(--tw-border-opacity))",
})
;"##### ; "390")]
#[test_case(r#####"tw`border-x-indigo-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(238 242 255 / var(--tw-border-opacity))",
  borderRightColor: "rgb(238 242 255 / var(--tw-border-opacity))",
})
;"##### ; "391")]
#[test_case(r#####"tw`border-x-indigo-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(224 231 255 / var(--tw-border-opacity))",
  borderRightColor: "rgb(224 231 255 / var(--tw-border-opacity))",
})
;"##### ; "392")]
#[test_case(r#####"tw`border-x-indigo-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(199 210 254 / var(--tw-border-opacity))",
  borderRightColor: "rgb(199 210 254 / var(--tw-border-opacity))",
})
;"##### ; "393")]
#[test_case(r#####"tw`border-x-indigo-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(165 180 252 / var(--tw-border-opacity))",
  borderRightColor: "rgb(165 180 252 / var(--tw-border-opacity))",
})
;"##### ; "394")]
#[test_case(r#####"tw`border-x-indigo-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(129 140 248 / var(--tw-border-opacity))",
  borderRightColor: "rgb(129 140 248 / var(--tw-border-opacity))",
})
;"##### ; "395")]
#[test_case(r#####"tw`border-x-indigo-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(99 102 241 / var(--tw-border-opacity))",
  borderRightColor: "rgb(99 102 241 / var(--tw-border-opacity))",
})
;"##### ; "396")]
#[test_case(r#####"tw`border-x-indigo-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(79 70 229 / var(--tw-border-opacity))",
  borderRightColor: "rgb(79 70 229 / var(--tw-border-opacity))",
})
;"##### ; "397")]
#[test_case(r#####"tw`border-x-indigo-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(67 56 202 / var(--tw-border-opacity))",
  borderRightColor: "rgb(67 56 202 / var(--tw-border-opacity))",
})
;"##### ; "398")]
#[test_case(r#####"tw`border-x-indigo-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(55 48 163 / var(--tw-border-opacity))",
  borderRightColor: "rgb(55 48 163 / var(--tw-border-opacity))",
})
;"##### ; "399")]
#[test_case(r#####"tw`border-x-indigo-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(49 46 129 / var(--tw-border-opacity))",
  borderRightColor: "rgb(49 46 129 / var(--tw-border-opacity))",
})
;"##### ; "400")]
#[test_case(r#####"tw`border-x-violet-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(245 243 255 / var(--tw-border-opacity))",
  borderRightColor: "rgb(245 243 255 / var(--tw-border-opacity))",
})
;"##### ; "401")]
#[test_case(r#####"tw`border-x-violet-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(237 233 254 / var(--tw-border-opacity))",
  borderRightColor: "rgb(237 233 254 / var(--tw-border-opacity))",
})
;"##### ; "402")]
#[test_case(r#####"tw`border-x-violet-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(221 214 254 / var(--tw-border-opacity))",
  borderRightColor: "rgb(221 214 254 / var(--tw-border-opacity))",
})
;"##### ; "403")]
#[test_case(r#####"tw`border-x-violet-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(196 181 253 / var(--tw-border-opacity))",
  borderRightColor: "rgb(196 181 253 / var(--tw-border-opacity))",
})
;"##### ; "404")]
#[test_case(r#####"tw`border-x-violet-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(167 139 250 / var(--tw-border-opacity))",
  borderRightColor: "rgb(167 139 250 / var(--tw-border-opacity))",
})
;"##### ; "405")]
#[test_case(r#####"tw`border-x-violet-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(139 92 246 / var(--tw-border-opacity))",
  borderRightColor: "rgb(139 92 246 / var(--tw-border-opacity))",
})
;"##### ; "406")]
#[test_case(r#####"tw`border-x-violet-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(124 58 237 / var(--tw-border-opacity))",
  borderRightColor: "rgb(124 58 237 / var(--tw-border-opacity))",
})
;"##### ; "407")]
#[test_case(r#####"tw`border-x-violet-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(109 40 217 / var(--tw-border-opacity))",
  borderRightColor: "rgb(109 40 217 / var(--tw-border-opacity))",
})
;"##### ; "408")]
#[test_case(r#####"tw`border-x-violet-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(91 33 182 / var(--tw-border-opacity))",
  borderRightColor: "rgb(91 33 182 / var(--tw-border-opacity))",
})
;"##### ; "409")]
#[test_case(r#####"tw`border-x-violet-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(76 29 149 / var(--tw-border-opacity))",
  borderRightColor: "rgb(76 29 149 / var(--tw-border-opacity))",
})
;"##### ; "410")]
#[test_case(r#####"tw`border-x-purple-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(250 245 255 / var(--tw-border-opacity))",
  borderRightColor: "rgb(250 245 255 / var(--tw-border-opacity))",
})
;"##### ; "411")]
#[test_case(r#####"tw`border-x-purple-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(243 232 255 / var(--tw-border-opacity))",
  borderRightColor: "rgb(243 232 255 / var(--tw-border-opacity))",
})
;"##### ; "412")]
#[test_case(r#####"tw`border-x-purple-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(233 213 255 / var(--tw-border-opacity))",
  borderRightColor: "rgb(233 213 255 / var(--tw-border-opacity))",
})
;"##### ; "413")]
#[test_case(r#####"tw`border-x-purple-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(216 180 254 / var(--tw-border-opacity))",
  borderRightColor: "rgb(216 180 254 / var(--tw-border-opacity))",
})
;"##### ; "414")]
#[test_case(r#####"tw`border-x-purple-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(192 132 252 / var(--tw-border-opacity))",
  borderRightColor: "rgb(192 132 252 / var(--tw-border-opacity))",
})
;"##### ; "415")]
#[test_case(r#####"tw`border-x-purple-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(168 85 247 / var(--tw-border-opacity))",
  borderRightColor: "rgb(168 85 247 / var(--tw-border-opacity))",
})
;"##### ; "416")]
#[test_case(r#####"tw`border-x-purple-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(147 51 234 / var(--tw-border-opacity))",
  borderRightColor: "rgb(147 51 234 / var(--tw-border-opacity))",
})
;"##### ; "417")]
#[test_case(r#####"tw`border-x-purple-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(126 34 206 / var(--tw-border-opacity))",
  borderRightColor: "rgb(126 34 206 / var(--tw-border-opacity))",
})
;"##### ; "418")]
#[test_case(r#####"tw`border-x-purple-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(107 33 168 / var(--tw-border-opacity))",
  borderRightColor: "rgb(107 33 168 / var(--tw-border-opacity))",
})
;"##### ; "419")]
#[test_case(r#####"tw`border-x-purple-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(88 28 135 / var(--tw-border-opacity))",
  borderRightColor: "rgb(88 28 135 / var(--tw-border-opacity))",
})
;"##### ; "420")]
#[test_case(r#####"tw`border-x-fuchsia-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(253 244 255 / var(--tw-border-opacity))",
  borderRightColor: "rgb(253 244 255 / var(--tw-border-opacity))",
})
;"##### ; "421")]
#[test_case(r#####"tw`border-x-fuchsia-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(250 232 255 / var(--tw-border-opacity))",
  borderRightColor: "rgb(250 232 255 / var(--tw-border-opacity))",
})
;"##### ; "422")]
#[test_case(r#####"tw`border-x-fuchsia-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(245 208 254 / var(--tw-border-opacity))",
  borderRightColor: "rgb(245 208 254 / var(--tw-border-opacity))",
})
;"##### ; "423")]
#[test_case(r#####"tw`border-x-fuchsia-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(240 171 252 / var(--tw-border-opacity))",
  borderRightColor: "rgb(240 171 252 / var(--tw-border-opacity))",
})
;"##### ; "424")]
#[test_case(r#####"tw`border-x-fuchsia-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(232 121 249 / var(--tw-border-opacity))",
  borderRightColor: "rgb(232 121 249 / var(--tw-border-opacity))",
})
;"##### ; "425")]
#[test_case(r#####"tw`border-x-fuchsia-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(217 70 239 / var(--tw-border-opacity))",
  borderRightColor: "rgb(217 70 239 / var(--tw-border-opacity))",
})
;"##### ; "426")]
#[test_case(r#####"tw`border-x-fuchsia-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(192 38 211 / var(--tw-border-opacity))",
  borderRightColor: "rgb(192 38 211 / var(--tw-border-opacity))",
})
;"##### ; "427")]
#[test_case(r#####"tw`border-x-fuchsia-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(162 28 175 / var(--tw-border-opacity))",
  borderRightColor: "rgb(162 28 175 / var(--tw-border-opacity))",
})
;"##### ; "428")]
#[test_case(r#####"tw`border-x-fuchsia-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(134 25 143 / var(--tw-border-opacity))",
  borderRightColor: "rgb(134 25 143 / var(--tw-border-opacity))",
})
;"##### ; "429")]
#[test_case(r#####"tw`border-x-fuchsia-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(112 26 117 / var(--tw-border-opacity))",
  borderRightColor: "rgb(112 26 117 / var(--tw-border-opacity))",
})
;"##### ; "430")]
#[test_case(r#####"tw`border-x-pink-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(253 242 248 / var(--tw-border-opacity))",
  borderRightColor: "rgb(253 242 248 / var(--tw-border-opacity))",
})
;"##### ; "431")]
#[test_case(r#####"tw`border-x-pink-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(252 231 243 / var(--tw-border-opacity))",
  borderRightColor: "rgb(252 231 243 / var(--tw-border-opacity))",
})
;"##### ; "432")]
#[test_case(r#####"tw`border-x-pink-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(251 207 232 / var(--tw-border-opacity))",
  borderRightColor: "rgb(251 207 232 / var(--tw-border-opacity))",
})
;"##### ; "433")]
#[test_case(r#####"tw`border-x-pink-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(249 168 212 / var(--tw-border-opacity))",
  borderRightColor: "rgb(249 168 212 / var(--tw-border-opacity))",
})
;"##### ; "434")]
#[test_case(r#####"tw`border-x-pink-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(244 114 182 / var(--tw-border-opacity))",
  borderRightColor: "rgb(244 114 182 / var(--tw-border-opacity))",
})
;"##### ; "435")]
#[test_case(r#####"tw`border-x-pink-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(236 72 153 / var(--tw-border-opacity))",
  borderRightColor: "rgb(236 72 153 / var(--tw-border-opacity))",
})
;"##### ; "436")]
#[test_case(r#####"tw`border-x-pink-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(219 39 119 / var(--tw-border-opacity))",
  borderRightColor: "rgb(219 39 119 / var(--tw-border-opacity))",
})
;"##### ; "437")]
#[test_case(r#####"tw`border-x-pink-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(190 24 93 / var(--tw-border-opacity))",
  borderRightColor: "rgb(190 24 93 / var(--tw-border-opacity))",
})
;"##### ; "438")]
#[test_case(r#####"tw`border-x-pink-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(157 23 77 / var(--tw-border-opacity))",
  borderRightColor: "rgb(157 23 77 / var(--tw-border-opacity))",
})
;"##### ; "439")]
#[test_case(r#####"tw`border-x-pink-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(131 24 67 / var(--tw-border-opacity))",
  borderRightColor: "rgb(131 24 67 / var(--tw-border-opacity))",
})
;"##### ; "440")]
#[test_case(r#####"tw`border-x-rose-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(255 241 242 / var(--tw-border-opacity))",
  borderRightColor: "rgb(255 241 242 / var(--tw-border-opacity))",
})
;"##### ; "441")]
#[test_case(r#####"tw`border-x-rose-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(255 228 230 / var(--tw-border-opacity))",
  borderRightColor: "rgb(255 228 230 / var(--tw-border-opacity))",
})
;"##### ; "442")]
#[test_case(r#####"tw`border-x-rose-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 205 211 / var(--tw-border-opacity))",
  borderRightColor: "rgb(254 205 211 / var(--tw-border-opacity))",
})
;"##### ; "443")]
#[test_case(r#####"tw`border-x-rose-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(253 164 175 / var(--tw-border-opacity))",
  borderRightColor: "rgb(253 164 175 / var(--tw-border-opacity))",
})
;"##### ; "444")]
#[test_case(r#####"tw`border-x-rose-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(251 113 133 / var(--tw-border-opacity))",
  borderRightColor: "rgb(251 113 133 / var(--tw-border-opacity))",
})
;"##### ; "445")]
#[test_case(r#####"tw`border-x-rose-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(244 63 94 / var(--tw-border-opacity))",
  borderRightColor: "rgb(244 63 94 / var(--tw-border-opacity))",
})
;"##### ; "446")]
#[test_case(r#####"tw`border-x-rose-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(225 29 72 / var(--tw-border-opacity))",
  borderRightColor: "rgb(225 29 72 / var(--tw-border-opacity))",
})
;"##### ; "447")]
#[test_case(r#####"tw`border-x-rose-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(190 18 60 / var(--tw-border-opacity))",
  borderRightColor: "rgb(190 18 60 / var(--tw-border-opacity))",
})
;"##### ; "448")]
#[test_case(r#####"tw`border-x-rose-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(159 18 57 / var(--tw-border-opacity))",
  borderRightColor: "rgb(159 18 57 / var(--tw-border-opacity))",
})
;"##### ; "449")]
#[test_case(r#####"tw`border-x-rose-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(136 19 55 / var(--tw-border-opacity))",
  borderRightColor: "rgb(136 19 55 / var(--tw-border-opacity))",
})
;"##### ; "450")]
#[test_case(r#####"tw`border-y-inherit`"#####, r#####"({
  borderTopColor: "inherit",
  borderBottomColor: "inherit",
})
;"##### ; "451")]
#[test_case(r#####"tw`border-y-current`"#####, r#####"({
  borderTopColor: "currentColor",
  borderBottomColor: "currentColor",
})
;"##### ; "452")]
#[test_case(r#####"tw`border-y-transparent`"#####, r#####"({
  borderTopColor: "transparent",
  borderBottomColor: "transparent",
})
;"##### ; "453")]
#[test_case(r#####"tw`border-y-black`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(0 0 0 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(0 0 0 / var(--tw-border-opacity))",
})
;"##### ; "454")]
#[test_case(r#####"tw`border-y-white`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 255 255 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(255 255 255 / var(--tw-border-opacity))",
})
;"##### ; "455")]
#[test_case(r#####"tw`border-y-slate-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(248 250 252 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(248 250 252 / var(--tw-border-opacity))",
})
;"##### ; "456")]
#[test_case(r#####"tw`border-y-slate-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(241 245 249 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(241 245 249 / var(--tw-border-opacity))",
})
;"##### ; "457")]
#[test_case(r#####"tw`border-y-slate-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(226 232 240 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(226 232 240 / var(--tw-border-opacity))",
})
;"##### ; "458")]
#[test_case(r#####"tw`border-y-slate-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(203 213 225 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(203 213 225 / var(--tw-border-opacity))",
})
;"##### ; "459")]
#[test_case(r#####"tw`border-y-slate-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(148 163 184 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(148 163 184 / var(--tw-border-opacity))",
})
;"##### ; "460")]
#[test_case(r#####"tw`border-y-slate-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(100 116 139 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(100 116 139 / var(--tw-border-opacity))",
})
;"##### ; "461")]
#[test_case(r#####"tw`border-y-slate-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(71 85 105 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(71 85 105 / var(--tw-border-opacity))",
})
;"##### ; "462")]
#[test_case(r#####"tw`border-y-slate-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(51 65 85 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(51 65 85 / var(--tw-border-opacity))",
})
;"##### ; "463")]
#[test_case(r#####"tw`border-y-slate-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(30 41 59 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(30 41 59 / var(--tw-border-opacity))",
})
;"##### ; "464")]
#[test_case(r#####"tw`border-y-slate-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(15 23 42 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(15 23 42 / var(--tw-border-opacity))",
})
;"##### ; "465")]
#[test_case(r#####"tw`border-y-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(249 250 251 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "466")]
#[test_case(r#####"tw`border-y-gray-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(243 244 246 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(243 244 246 / var(--tw-border-opacity))",
})
;"##### ; "467")]
#[test_case(r#####"tw`border-y-gray-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(229 231 235 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(229 231 235 / var(--tw-border-opacity))",
})
;"##### ; "468")]
#[test_case(r#####"tw`border-y-gray-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(209 213 219 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(209 213 219 / var(--tw-border-opacity))",
})
;"##### ; "469")]
#[test_case(r#####"tw`border-y-gray-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(156 163 175 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(156 163 175 / var(--tw-border-opacity))",
})
;"##### ; "470")]
#[test_case(r#####"tw`border-y-gray-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(107 114 128 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(107 114 128 / var(--tw-border-opacity))",
})
;"##### ; "471")]
#[test_case(r#####"tw`border-y-gray-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(75 85 99 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(75 85 99 / var(--tw-border-opacity))",
})
;"##### ; "472")]
#[test_case(r#####"tw`border-y-gray-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(55 65 81 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(55 65 81 / var(--tw-border-opacity))",
})
;"##### ; "473")]
#[test_case(r#####"tw`border-y-gray-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(31 41 55 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(31 41 55 / var(--tw-border-opacity))",
})
;"##### ; "474")]
#[test_case(r#####"tw`border-y-gray-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(17 24 39 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(17 24 39 / var(--tw-border-opacity))",
})
;"##### ; "475")]
#[test_case(r#####"tw`border-y-zinc-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(250 250 250 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "476")]
#[test_case(r#####"tw`border-y-zinc-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(244 244 245 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(244 244 245 / var(--tw-border-opacity))",
})
;"##### ; "477")]
#[test_case(r#####"tw`border-y-zinc-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(228 228 231 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(228 228 231 / var(--tw-border-opacity))",
})
;"##### ; "478")]
#[test_case(r#####"tw`border-y-zinc-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(212 212 216 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(212 212 216 / var(--tw-border-opacity))",
})
;"##### ; "479")]
#[test_case(r#####"tw`border-y-zinc-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(161 161 170 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(161 161 170 / var(--tw-border-opacity))",
})
;"##### ; "480")]
#[test_case(r#####"tw`border-y-zinc-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(113 113 122 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(113 113 122 / var(--tw-border-opacity))",
})
;"##### ; "481")]
#[test_case(r#####"tw`border-y-zinc-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(82 82 91 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(82 82 91 / var(--tw-border-opacity))",
})
;"##### ; "482")]
#[test_case(r#####"tw`border-y-zinc-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(63 63 70 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(63 63 70 / var(--tw-border-opacity))",
})
;"##### ; "483")]
#[test_case(r#####"tw`border-y-zinc-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(39 39 42 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(39 39 42 / var(--tw-border-opacity))",
})
;"##### ; "484")]
#[test_case(r#####"tw`border-y-zinc-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(24 24 27 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(24 24 27 / var(--tw-border-opacity))",
})
;"##### ; "485")]
#[test_case(r#####"tw`border-y-neutral-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(250 250 250 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "486")]
#[test_case(r#####"tw`border-y-neutral-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(245 245 245 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(245 245 245 / var(--tw-border-opacity))",
})
;"##### ; "487")]
#[test_case(r#####"tw`border-y-neutral-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(229 229 229 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(229 229 229 / var(--tw-border-opacity))",
})
;"##### ; "488")]
#[test_case(r#####"tw`border-y-neutral-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(212 212 212 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(212 212 212 / var(--tw-border-opacity))",
})
;"##### ; "489")]
#[test_case(r#####"tw`border-y-neutral-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(163 163 163 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(163 163 163 / var(--tw-border-opacity))",
})
;"##### ; "490")]
#[test_case(r#####"tw`border-y-neutral-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(115 115 115 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(115 115 115 / var(--tw-border-opacity))",
})
;"##### ; "491")]
#[test_case(r#####"tw`border-y-neutral-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(82 82 82 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(82 82 82 / var(--tw-border-opacity))",
})
;"##### ; "492")]
#[test_case(r#####"tw`border-y-neutral-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(64 64 64 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(64 64 64 / var(--tw-border-opacity))",
})
;"##### ; "493")]
#[test_case(r#####"tw`border-y-neutral-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(38 38 38 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(38 38 38 / var(--tw-border-opacity))",
})
;"##### ; "494")]
#[test_case(r#####"tw`border-y-neutral-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(23 23 23 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(23 23 23 / var(--tw-border-opacity))",
})
;"##### ; "495")]
#[test_case(r#####"tw`border-y-stone-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(250 250 249 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(250 250 249 / var(--tw-border-opacity))",
})
;"##### ; "496")]
#[test_case(r#####"tw`border-y-stone-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(245 245 244 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(245 245 244 / var(--tw-border-opacity))",
})
;"##### ; "497")]
#[test_case(r#####"tw`border-y-stone-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(231 229 228 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(231 229 228 / var(--tw-border-opacity))",
})
;"##### ; "498")]
#[test_case(r#####"tw`border-y-stone-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(214 211 209 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(214 211 209 / var(--tw-border-opacity))",
})
;"##### ; "499")]
#[test_case(r#####"tw`border-y-stone-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(168 162 158 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(168 162 158 / var(--tw-border-opacity))",
})
;"##### ; "500")]
#[test_case(r#####"tw`border-y-stone-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(120 113 108 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(120 113 108 / var(--tw-border-opacity))",
})
;"##### ; "501")]
#[test_case(r#####"tw`border-y-stone-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(87 83 78 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(87 83 78 / var(--tw-border-opacity))",
})
;"##### ; "502")]
#[test_case(r#####"tw`border-y-stone-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(68 64 60 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(68 64 60 / var(--tw-border-opacity))",
})
;"##### ; "503")]
#[test_case(r#####"tw`border-y-stone-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(41 37 36 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(41 37 36 / var(--tw-border-opacity))",
})
;"##### ; "504")]
#[test_case(r#####"tw`border-y-stone-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(28 25 23 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(28 25 23 / var(--tw-border-opacity))",
})
;"##### ; "505")]
#[test_case(r#####"tw`border-y-red-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 242 242 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(254 242 242 / var(--tw-border-opacity))",
})
;"##### ; "506")]
#[test_case(r#####"tw`border-y-red-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 226 226 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(254 226 226 / var(--tw-border-opacity))",
})
;"##### ; "507")]
#[test_case(r#####"tw`border-y-red-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 202 202 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(254 202 202 / var(--tw-border-opacity))",
})
;"##### ; "508")]
#[test_case(r#####"tw`border-y-red-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(252 165 165 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(252 165 165 / var(--tw-border-opacity))",
})
;"##### ; "509")]
#[test_case(r#####"tw`border-y-red-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(248 113 113 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(248 113 113 / var(--tw-border-opacity))",
})
;"##### ; "510")]
#[test_case(r#####"tw`border-y-red-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(239 68 68 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(239 68 68 / var(--tw-border-opacity))",
})
;"##### ; "511")]
#[test_case(r#####"tw`border-y-red-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(220 38 38 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(220 38 38 / var(--tw-border-opacity))",
})
;"##### ; "512")]
#[test_case(r#####"tw`border-y-red-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(185 28 28 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(185 28 28 / var(--tw-border-opacity))",
})
;"##### ; "513")]
#[test_case(r#####"tw`border-y-red-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(153 27 27 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(153 27 27 / var(--tw-border-opacity))",
})
;"##### ; "514")]
#[test_case(r#####"tw`border-y-red-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(127 29 29 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(127 29 29 / var(--tw-border-opacity))",
})
;"##### ; "515")]
#[test_case(r#####"tw`border-y-orange-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 247 237 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(255 247 237 / var(--tw-border-opacity))",
})
;"##### ; "516")]
#[test_case(r#####"tw`border-y-orange-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 237 213 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(255 237 213 / var(--tw-border-opacity))",
})
;"##### ; "517")]
#[test_case(r#####"tw`border-y-orange-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 215 170 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(254 215 170 / var(--tw-border-opacity))",
})
;"##### ; "518")]
#[test_case(r#####"tw`border-y-orange-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(253 186 116 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(253 186 116 / var(--tw-border-opacity))",
})
;"##### ; "519")]
#[test_case(r#####"tw`border-y-orange-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(251 146 60 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(251 146 60 / var(--tw-border-opacity))",
})
;"##### ; "520")]
#[test_case(r#####"tw`border-y-orange-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(249 115 22 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(249 115 22 / var(--tw-border-opacity))",
})
;"##### ; "521")]
#[test_case(r#####"tw`border-y-orange-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(234 88 12 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(234 88 12 / var(--tw-border-opacity))",
})
;"##### ; "522")]
#[test_case(r#####"tw`border-y-orange-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(194 65 12 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(194 65 12 / var(--tw-border-opacity))",
})
;"##### ; "523")]
#[test_case(r#####"tw`border-y-orange-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(154 52 18 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(154 52 18 / var(--tw-border-opacity))",
})
;"##### ; "524")]
#[test_case(r#####"tw`border-y-orange-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(124 45 18 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(124 45 18 / var(--tw-border-opacity))",
})
;"##### ; "525")]
#[test_case(r#####"tw`border-y-amber-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 251 235 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(255 251 235 / var(--tw-border-opacity))",
})
;"##### ; "526")]
#[test_case(r#####"tw`border-y-amber-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 243 199 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(254 243 199 / var(--tw-border-opacity))",
})
;"##### ; "527")]
#[test_case(r#####"tw`border-y-amber-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(253 230 138 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(253 230 138 / var(--tw-border-opacity))",
})
;"##### ; "528")]
#[test_case(r#####"tw`border-y-amber-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(252 211 77 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(252 211 77 / var(--tw-border-opacity))",
})
;"##### ; "529")]
#[test_case(r#####"tw`border-y-amber-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(251 191 36 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(251 191 36 / var(--tw-border-opacity))",
})
;"##### ; "530")]
#[test_case(r#####"tw`border-y-amber-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(245 158 11 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(245 158 11 / var(--tw-border-opacity))",
})
;"##### ; "531")]
#[test_case(r#####"tw`border-y-amber-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(217 119 6 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(217 119 6 / var(--tw-border-opacity))",
})
;"##### ; "532")]
#[test_case(r#####"tw`border-y-amber-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(180 83 9 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(180 83 9 / var(--tw-border-opacity))",
})
;"##### ; "533")]
#[test_case(r#####"tw`border-y-amber-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(146 64 14 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(146 64 14 / var(--tw-border-opacity))",
})
;"##### ; "534")]
#[test_case(r#####"tw`border-y-amber-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(120 53 15 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(120 53 15 / var(--tw-border-opacity))",
})
;"##### ; "535")]
#[test_case(r#####"tw`border-y-yellow-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 252 232 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(254 252 232 / var(--tw-border-opacity))",
})
;"##### ; "536")]
#[test_case(r#####"tw`border-y-yellow-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 249 195 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(254 249 195 / var(--tw-border-opacity))",
})
;"##### ; "537")]
#[test_case(r#####"tw`border-y-yellow-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 240 138 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(254 240 138 / var(--tw-border-opacity))",
})
;"##### ; "538")]
#[test_case(r#####"tw`border-y-yellow-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(253 224 71 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(253 224 71 / var(--tw-border-opacity))",
})
;"##### ; "539")]
#[test_case(r#####"tw`border-y-yellow-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(250 204 21 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(250 204 21 / var(--tw-border-opacity))",
})
;"##### ; "540")]
#[test_case(r#####"tw`border-y-yellow-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(234 179 8 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(234 179 8 / var(--tw-border-opacity))",
})
;"##### ; "541")]
#[test_case(r#####"tw`border-y-yellow-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(202 138 4 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(202 138 4 / var(--tw-border-opacity))",
})
;"##### ; "542")]
#[test_case(r#####"tw`border-y-yellow-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(161 98 7 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(161 98 7 / var(--tw-border-opacity))",
})
;"##### ; "543")]
#[test_case(r#####"tw`border-y-yellow-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(133 77 14 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(133 77 14 / var(--tw-border-opacity))",
})
;"##### ; "544")]
#[test_case(r#####"tw`border-y-yellow-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(113 63 18 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(113 63 18 / var(--tw-border-opacity))",
})
;"##### ; "545")]
#[test_case(r#####"tw`border-y-lime-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(247 254 231 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(247 254 231 / var(--tw-border-opacity))",
})
;"##### ; "546")]
#[test_case(r#####"tw`border-y-lime-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(236 252 203 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(236 252 203 / var(--tw-border-opacity))",
})
;"##### ; "547")]
#[test_case(r#####"tw`border-y-lime-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(217 249 157 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(217 249 157 / var(--tw-border-opacity))",
})
;"##### ; "548")]
#[test_case(r#####"tw`border-y-lime-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(190 242 100 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(190 242 100 / var(--tw-border-opacity))",
})
;"##### ; "549")]
#[test_case(r#####"tw`border-y-lime-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(163 230 53 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(163 230 53 / var(--tw-border-opacity))",
})
;"##### ; "550")]
#[test_case(r#####"tw`border-y-lime-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(132 204 22 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(132 204 22 / var(--tw-border-opacity))",
})
;"##### ; "551")]
#[test_case(r#####"tw`border-y-lime-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(101 163 13 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(101 163 13 / var(--tw-border-opacity))",
})
;"##### ; "552")]
#[test_case(r#####"tw`border-y-lime-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(77 124 15 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(77 124 15 / var(--tw-border-opacity))",
})
;"##### ; "553")]
#[test_case(r#####"tw`border-y-lime-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(63 98 18 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(63 98 18 / var(--tw-border-opacity))",
})
;"##### ; "554")]
#[test_case(r#####"tw`border-y-lime-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(54 83 20 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(54 83 20 / var(--tw-border-opacity))",
})
;"##### ; "555")]
#[test_case(r#####"tw`border-y-green-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(240 253 244 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(240 253 244 / var(--tw-border-opacity))",
})
;"##### ; "556")]
#[test_case(r#####"tw`border-y-green-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(220 252 231 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(220 252 231 / var(--tw-border-opacity))",
})
;"##### ; "557")]
#[test_case(r#####"tw`border-y-green-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(187 247 208 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(187 247 208 / var(--tw-border-opacity))",
})
;"##### ; "558")]
#[test_case(r#####"tw`border-y-green-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(134 239 172 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(134 239 172 / var(--tw-border-opacity))",
})
;"##### ; "559")]
#[test_case(r#####"tw`border-y-green-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(74 222 128 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(74 222 128 / var(--tw-border-opacity))",
})
;"##### ; "560")]
#[test_case(r#####"tw`border-y-green-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(34 197 94 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(34 197 94 / var(--tw-border-opacity))",
})
;"##### ; "561")]
#[test_case(r#####"tw`border-y-green-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(22 163 74 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(22 163 74 / var(--tw-border-opacity))",
})
;"##### ; "562")]
#[test_case(r#####"tw`border-y-green-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(21 128 61 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(21 128 61 / var(--tw-border-opacity))",
})
;"##### ; "563")]
#[test_case(r#####"tw`border-y-green-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(22 101 52 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(22 101 52 / var(--tw-border-opacity))",
})
;"##### ; "564")]
#[test_case(r#####"tw`border-y-green-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(20 83 45 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(20 83 45 / var(--tw-border-opacity))",
})
;"##### ; "565")]
#[test_case(r#####"tw`border-y-emerald-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(236 253 245 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(236 253 245 / var(--tw-border-opacity))",
})
;"##### ; "566")]
#[test_case(r#####"tw`border-y-emerald-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(209 250 229 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(209 250 229 / var(--tw-border-opacity))",
})
;"##### ; "567")]
#[test_case(r#####"tw`border-y-emerald-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(167 243 208 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(167 243 208 / var(--tw-border-opacity))",
})
;"##### ; "568")]
#[test_case(r#####"tw`border-y-emerald-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(110 231 183 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(110 231 183 / var(--tw-border-opacity))",
})
;"##### ; "569")]
#[test_case(r#####"tw`border-y-emerald-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(52 211 153 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(52 211 153 / var(--tw-border-opacity))",
})
;"##### ; "570")]
#[test_case(r#####"tw`border-y-emerald-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(16 185 129 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(16 185 129 / var(--tw-border-opacity))",
})
;"##### ; "571")]
#[test_case(r#####"tw`border-y-emerald-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(5 150 105 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(5 150 105 / var(--tw-border-opacity))",
})
;"##### ; "572")]
#[test_case(r#####"tw`border-y-emerald-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(4 120 87 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(4 120 87 / var(--tw-border-opacity))",
})
;"##### ; "573")]
#[test_case(r#####"tw`border-y-emerald-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(6 95 70 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(6 95 70 / var(--tw-border-opacity))",
})
;"##### ; "574")]
#[test_case(r#####"tw`border-y-emerald-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(6 78 59 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(6 78 59 / var(--tw-border-opacity))",
})
;"##### ; "575")]
#[test_case(r#####"tw`border-y-teal-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(240 253 250 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(240 253 250 / var(--tw-border-opacity))",
})
;"##### ; "576")]
#[test_case(r#####"tw`border-y-teal-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(204 251 241 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(204 251 241 / var(--tw-border-opacity))",
})
;"##### ; "577")]
#[test_case(r#####"tw`border-y-teal-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(153 246 228 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(153 246 228 / var(--tw-border-opacity))",
})
;"##### ; "578")]
#[test_case(r#####"tw`border-y-teal-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(94 234 212 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(94 234 212 / var(--tw-border-opacity))",
})
;"##### ; "579")]
#[test_case(r#####"tw`border-y-teal-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(45 212 191 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(45 212 191 / var(--tw-border-opacity))",
})
;"##### ; "580")]
#[test_case(r#####"tw`border-y-teal-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(20 184 166 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(20 184 166 / var(--tw-border-opacity))",
})
;"##### ; "581")]
#[test_case(r#####"tw`border-y-teal-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(13 148 136 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(13 148 136 / var(--tw-border-opacity))",
})
;"##### ; "582")]
#[test_case(r#####"tw`border-y-teal-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(15 118 110 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(15 118 110 / var(--tw-border-opacity))",
})
;"##### ; "583")]
#[test_case(r#####"tw`border-y-teal-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(17 94 89 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(17 94 89 / var(--tw-border-opacity))",
})
;"##### ; "584")]
#[test_case(r#####"tw`border-y-teal-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(19 78 74 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(19 78 74 / var(--tw-border-opacity))",
})
;"##### ; "585")]
#[test_case(r#####"tw`border-y-cyan-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(236 254 255 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(236 254 255 / var(--tw-border-opacity))",
})
;"##### ; "586")]
#[test_case(r#####"tw`border-y-cyan-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(207 250 254 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(207 250 254 / var(--tw-border-opacity))",
})
;"##### ; "587")]
#[test_case(r#####"tw`border-y-cyan-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(165 243 252 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(165 243 252 / var(--tw-border-opacity))",
})
;"##### ; "588")]
#[test_case(r#####"tw`border-y-cyan-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(103 232 249 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(103 232 249 / var(--tw-border-opacity))",
})
;"##### ; "589")]
#[test_case(r#####"tw`border-y-cyan-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(34 211 238 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(34 211 238 / var(--tw-border-opacity))",
})
;"##### ; "590")]
#[test_case(r#####"tw`border-y-cyan-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(6 182 212 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(6 182 212 / var(--tw-border-opacity))",
})
;"##### ; "591")]
#[test_case(r#####"tw`border-y-cyan-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(8 145 178 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(8 145 178 / var(--tw-border-opacity))",
})
;"##### ; "592")]
#[test_case(r#####"tw`border-y-cyan-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(14 116 144 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(14 116 144 / var(--tw-border-opacity))",
})
;"##### ; "593")]
#[test_case(r#####"tw`border-y-cyan-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(21 94 117 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(21 94 117 / var(--tw-border-opacity))",
})
;"##### ; "594")]
#[test_case(r#####"tw`border-y-cyan-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(22 78 99 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(22 78 99 / var(--tw-border-opacity))",
})
;"##### ; "595")]
#[test_case(r#####"tw`border-y-sky-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(240 249 255 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(240 249 255 / var(--tw-border-opacity))",
})
;"##### ; "596")]
#[test_case(r#####"tw`border-y-sky-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(224 242 254 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(224 242 254 / var(--tw-border-opacity))",
})
;"##### ; "597")]
#[test_case(r#####"tw`border-y-sky-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(186 230 253 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(186 230 253 / var(--tw-border-opacity))",
})
;"##### ; "598")]
#[test_case(r#####"tw`border-y-sky-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(125 211 252 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(125 211 252 / var(--tw-border-opacity))",
})
;"##### ; "599")]
#[test_case(r#####"tw`border-y-sky-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(56 189 248 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(56 189 248 / var(--tw-border-opacity))",
})
;"##### ; "600")]
#[test_case(r#####"tw`border-y-sky-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(14 165 233 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(14 165 233 / var(--tw-border-opacity))",
})
;"##### ; "601")]
#[test_case(r#####"tw`border-y-sky-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(2 132 199 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(2 132 199 / var(--tw-border-opacity))",
})
;"##### ; "602")]
#[test_case(r#####"tw`border-y-sky-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(3 105 161 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(3 105 161 / var(--tw-border-opacity))",
})
;"##### ; "603")]
#[test_case(r#####"tw`border-y-sky-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(7 89 133 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(7 89 133 / var(--tw-border-opacity))",
})
;"##### ; "604")]
#[test_case(r#####"tw`border-y-sky-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(12 74 110 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(12 74 110 / var(--tw-border-opacity))",
})
;"##### ; "605")]
#[test_case(r#####"tw`border-y-blue-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(239 246 255 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(239 246 255 / var(--tw-border-opacity))",
})
;"##### ; "606")]
#[test_case(r#####"tw`border-y-blue-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(219 234 254 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(219 234 254 / var(--tw-border-opacity))",
})
;"##### ; "607")]
#[test_case(r#####"tw`border-y-blue-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(191 219 254 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(191 219 254 / var(--tw-border-opacity))",
})
;"##### ; "608")]
#[test_case(r#####"tw`border-y-blue-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(147 197 253 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(147 197 253 / var(--tw-border-opacity))",
})
;"##### ; "609")]
#[test_case(r#####"tw`border-y-blue-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(96 165 250 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(96 165 250 / var(--tw-border-opacity))",
})
;"##### ; "610")]
#[test_case(r#####"tw`border-y-blue-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(59 130 246 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(59 130 246 / var(--tw-border-opacity))",
})
;"##### ; "611")]
#[test_case(r#####"tw`border-y-blue-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(37 99 235 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(37 99 235 / var(--tw-border-opacity))",
})
;"##### ; "612")]
#[test_case(r#####"tw`border-y-blue-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(29 78 216 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(29 78 216 / var(--tw-border-opacity))",
})
;"##### ; "613")]
#[test_case(r#####"tw`border-y-blue-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(30 64 175 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(30 64 175 / var(--tw-border-opacity))",
})
;"##### ; "614")]
#[test_case(r#####"tw`border-y-blue-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(30 58 138 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(30 58 138 / var(--tw-border-opacity))",
})
;"##### ; "615")]
#[test_case(r#####"tw`border-y-indigo-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(238 242 255 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(238 242 255 / var(--tw-border-opacity))",
})
;"##### ; "616")]
#[test_case(r#####"tw`border-y-indigo-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(224 231 255 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(224 231 255 / var(--tw-border-opacity))",
})
;"##### ; "617")]
#[test_case(r#####"tw`border-y-indigo-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(199 210 254 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(199 210 254 / var(--tw-border-opacity))",
})
;"##### ; "618")]
#[test_case(r#####"tw`border-y-indigo-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(165 180 252 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(165 180 252 / var(--tw-border-opacity))",
})
;"##### ; "619")]
#[test_case(r#####"tw`border-y-indigo-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(129 140 248 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(129 140 248 / var(--tw-border-opacity))",
})
;"##### ; "620")]
#[test_case(r#####"tw`border-y-indigo-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(99 102 241 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(99 102 241 / var(--tw-border-opacity))",
})
;"##### ; "621")]
#[test_case(r#####"tw`border-y-indigo-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(79 70 229 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(79 70 229 / var(--tw-border-opacity))",
})
;"##### ; "622")]
#[test_case(r#####"tw`border-y-indigo-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(67 56 202 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(67 56 202 / var(--tw-border-opacity))",
})
;"##### ; "623")]
#[test_case(r#####"tw`border-y-indigo-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(55 48 163 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(55 48 163 / var(--tw-border-opacity))",
})
;"##### ; "624")]
#[test_case(r#####"tw`border-y-indigo-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(49 46 129 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(49 46 129 / var(--tw-border-opacity))",
})
;"##### ; "625")]
#[test_case(r#####"tw`border-y-violet-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(245 243 255 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(245 243 255 / var(--tw-border-opacity))",
})
;"##### ; "626")]
#[test_case(r#####"tw`border-y-violet-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(237 233 254 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(237 233 254 / var(--tw-border-opacity))",
})
;"##### ; "627")]
#[test_case(r#####"tw`border-y-violet-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(221 214 254 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(221 214 254 / var(--tw-border-opacity))",
})
;"##### ; "628")]
#[test_case(r#####"tw`border-y-violet-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(196 181 253 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(196 181 253 / var(--tw-border-opacity))",
})
;"##### ; "629")]
#[test_case(r#####"tw`border-y-violet-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(167 139 250 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(167 139 250 / var(--tw-border-opacity))",
})
;"##### ; "630")]
#[test_case(r#####"tw`border-y-violet-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(139 92 246 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(139 92 246 / var(--tw-border-opacity))",
})
;"##### ; "631")]
#[test_case(r#####"tw`border-y-violet-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(124 58 237 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(124 58 237 / var(--tw-border-opacity))",
})
;"##### ; "632")]
#[test_case(r#####"tw`border-y-violet-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(109 40 217 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(109 40 217 / var(--tw-border-opacity))",
})
;"##### ; "633")]
#[test_case(r#####"tw`border-y-violet-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(91 33 182 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(91 33 182 / var(--tw-border-opacity))",
})
;"##### ; "634")]
#[test_case(r#####"tw`border-y-violet-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(76 29 149 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(76 29 149 / var(--tw-border-opacity))",
})
;"##### ; "635")]
#[test_case(r#####"tw`border-y-purple-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(250 245 255 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(250 245 255 / var(--tw-border-opacity))",
})
;"##### ; "636")]
#[test_case(r#####"tw`border-y-purple-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(243 232 255 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(243 232 255 / var(--tw-border-opacity))",
})
;"##### ; "637")]
#[test_case(r#####"tw`border-y-purple-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(233 213 255 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(233 213 255 / var(--tw-border-opacity))",
})
;"##### ; "638")]
#[test_case(r#####"tw`border-y-purple-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(216 180 254 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(216 180 254 / var(--tw-border-opacity))",
})
;"##### ; "639")]
#[test_case(r#####"tw`border-y-purple-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(192 132 252 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(192 132 252 / var(--tw-border-opacity))",
})
;"##### ; "640")]
#[test_case(r#####"tw`border-y-purple-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(168 85 247 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(168 85 247 / var(--tw-border-opacity))",
})
;"##### ; "641")]
#[test_case(r#####"tw`border-y-purple-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(147 51 234 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(147 51 234 / var(--tw-border-opacity))",
})
;"##### ; "642")]
#[test_case(r#####"tw`border-y-purple-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(126 34 206 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(126 34 206 / var(--tw-border-opacity))",
})
;"##### ; "643")]
#[test_case(r#####"tw`border-y-purple-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(107 33 168 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(107 33 168 / var(--tw-border-opacity))",
})
;"##### ; "644")]
#[test_case(r#####"tw`border-y-purple-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(88 28 135 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(88 28 135 / var(--tw-border-opacity))",
})
;"##### ; "645")]
#[test_case(r#####"tw`border-y-fuchsia-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(253 244 255 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(253 244 255 / var(--tw-border-opacity))",
})
;"##### ; "646")]
#[test_case(r#####"tw`border-y-fuchsia-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(250 232 255 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(250 232 255 / var(--tw-border-opacity))",
})
;"##### ; "647")]
#[test_case(r#####"tw`border-y-fuchsia-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(245 208 254 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(245 208 254 / var(--tw-border-opacity))",
})
;"##### ; "648")]
#[test_case(r#####"tw`border-y-fuchsia-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(240 171 252 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(240 171 252 / var(--tw-border-opacity))",
})
;"##### ; "649")]
#[test_case(r#####"tw`border-y-fuchsia-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(232 121 249 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(232 121 249 / var(--tw-border-opacity))",
})
;"##### ; "650")]
#[test_case(r#####"tw`border-y-fuchsia-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(217 70 239 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(217 70 239 / var(--tw-border-opacity))",
})
;"##### ; "651")]
#[test_case(r#####"tw`border-y-fuchsia-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(192 38 211 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(192 38 211 / var(--tw-border-opacity))",
})
;"##### ; "652")]
#[test_case(r#####"tw`border-y-fuchsia-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(162 28 175 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(162 28 175 / var(--tw-border-opacity))",
})
;"##### ; "653")]
#[test_case(r#####"tw`border-y-fuchsia-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(134 25 143 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(134 25 143 / var(--tw-border-opacity))",
})
;"##### ; "654")]
#[test_case(r#####"tw`border-y-fuchsia-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(112 26 117 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(112 26 117 / var(--tw-border-opacity))",
})
;"##### ; "655")]
#[test_case(r#####"tw`border-y-pink-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(253 242 248 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(253 242 248 / var(--tw-border-opacity))",
})
;"##### ; "656")]
#[test_case(r#####"tw`border-y-pink-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(252 231 243 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(252 231 243 / var(--tw-border-opacity))",
})
;"##### ; "657")]
#[test_case(r#####"tw`border-y-pink-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(251 207 232 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(251 207 232 / var(--tw-border-opacity))",
})
;"##### ; "658")]
#[test_case(r#####"tw`border-y-pink-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(249 168 212 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(249 168 212 / var(--tw-border-opacity))",
})
;"##### ; "659")]
#[test_case(r#####"tw`border-y-pink-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(244 114 182 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(244 114 182 / var(--tw-border-opacity))",
})
;"##### ; "660")]
#[test_case(r#####"tw`border-y-pink-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(236 72 153 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(236 72 153 / var(--tw-border-opacity))",
})
;"##### ; "661")]
#[test_case(r#####"tw`border-y-pink-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(219 39 119 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(219 39 119 / var(--tw-border-opacity))",
})
;"##### ; "662")]
#[test_case(r#####"tw`border-y-pink-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(190 24 93 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(190 24 93 / var(--tw-border-opacity))",
})
;"##### ; "663")]
#[test_case(r#####"tw`border-y-pink-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(157 23 77 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(157 23 77 / var(--tw-border-opacity))",
})
;"##### ; "664")]
#[test_case(r#####"tw`border-y-pink-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(131 24 67 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(131 24 67 / var(--tw-border-opacity))",
})
;"##### ; "665")]
#[test_case(r#####"tw`border-y-rose-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 241 242 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(255 241 242 / var(--tw-border-opacity))",
})
;"##### ; "666")]
#[test_case(r#####"tw`border-y-rose-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 228 230 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(255 228 230 / var(--tw-border-opacity))",
})
;"##### ; "667")]
#[test_case(r#####"tw`border-y-rose-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 205 211 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(254 205 211 / var(--tw-border-opacity))",
})
;"##### ; "668")]
#[test_case(r#####"tw`border-y-rose-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(253 164 175 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(253 164 175 / var(--tw-border-opacity))",
})
;"##### ; "669")]
#[test_case(r#####"tw`border-y-rose-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(251 113 133 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(251 113 133 / var(--tw-border-opacity))",
})
;"##### ; "670")]
#[test_case(r#####"tw`border-y-rose-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(244 63 94 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(244 63 94 / var(--tw-border-opacity))",
})
;"##### ; "671")]
#[test_case(r#####"tw`border-y-rose-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(225 29 72 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(225 29 72 / var(--tw-border-opacity))",
})
;"##### ; "672")]
#[test_case(r#####"tw`border-y-rose-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(190 18 60 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(190 18 60 / var(--tw-border-opacity))",
})
;"##### ; "673")]
#[test_case(r#####"tw`border-y-rose-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(159 18 57 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(159 18 57 / var(--tw-border-opacity))",
})
;"##### ; "674")]
#[test_case(r#####"tw`border-y-rose-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(136 19 55 / var(--tw-border-opacity))",
  borderBottomColor: "rgb(136 19 55 / var(--tw-border-opacity))",
})
;"##### ; "675")]
#[test_case(r#####"tw`border-t-inherit`"#####, r#####"({
  borderTopColor: "inherit",
})
;"##### ; "676")]
#[test_case(r#####"tw`border-t-current`"#####, r#####"({
  borderTopColor: "currentColor",
})
;"##### ; "677")]
#[test_case(r#####"tw`border-t-transparent`"#####, r#####"({
  borderTopColor: "transparent",
})
;"##### ; "678")]
#[test_case(r#####"tw`border-t-black`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(0 0 0 / var(--tw-border-opacity))",
})
;"##### ; "679")]
#[test_case(r#####"tw`border-t-white`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 255 255 / var(--tw-border-opacity))",
})
;"##### ; "680")]
#[test_case(r#####"tw`border-t-slate-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(248 250 252 / var(--tw-border-opacity))",
})
;"##### ; "681")]
#[test_case(r#####"tw`border-t-slate-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(241 245 249 / var(--tw-border-opacity))",
})
;"##### ; "682")]
#[test_case(r#####"tw`border-t-slate-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(226 232 240 / var(--tw-border-opacity))",
})
;"##### ; "683")]
#[test_case(r#####"tw`border-t-slate-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(203 213 225 / var(--tw-border-opacity))",
})
;"##### ; "684")]
#[test_case(r#####"tw`border-t-slate-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(148 163 184 / var(--tw-border-opacity))",
})
;"##### ; "685")]
#[test_case(r#####"tw`border-t-slate-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(100 116 139 / var(--tw-border-opacity))",
})
;"##### ; "686")]
#[test_case(r#####"tw`border-t-slate-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(71 85 105 / var(--tw-border-opacity))",
})
;"##### ; "687")]
#[test_case(r#####"tw`border-t-slate-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(51 65 85 / var(--tw-border-opacity))",
})
;"##### ; "688")]
#[test_case(r#####"tw`border-t-slate-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(30 41 59 / var(--tw-border-opacity))",
})
;"##### ; "689")]
#[test_case(r#####"tw`border-t-slate-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(15 23 42 / var(--tw-border-opacity))",
})
;"##### ; "690")]
#[test_case(r#####"tw`border-t-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "691")]
#[test_case(r#####"tw`border-t-gray-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(243 244 246 / var(--tw-border-opacity))",
})
;"##### ; "692")]
#[test_case(r#####"tw`border-t-gray-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(229 231 235 / var(--tw-border-opacity))",
})
;"##### ; "693")]
#[test_case(r#####"tw`border-t-gray-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(209 213 219 / var(--tw-border-opacity))",
})
;"##### ; "694")]
#[test_case(r#####"tw`border-t-gray-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(156 163 175 / var(--tw-border-opacity))",
})
;"##### ; "695")]
#[test_case(r#####"tw`border-t-gray-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(107 114 128 / var(--tw-border-opacity))",
})
;"##### ; "696")]
#[test_case(r#####"tw`border-t-gray-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(75 85 99 / var(--tw-border-opacity))",
})
;"##### ; "697")]
#[test_case(r#####"tw`border-t-gray-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(55 65 81 / var(--tw-border-opacity))",
})
;"##### ; "698")]
#[test_case(r#####"tw`border-t-gray-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(31 41 55 / var(--tw-border-opacity))",
})
;"##### ; "699")]
#[test_case(r#####"tw`border-t-gray-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(17 24 39 / var(--tw-border-opacity))",
})
;"##### ; "700")]
#[test_case(r#####"tw`border-t-zinc-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "701")]
#[test_case(r#####"tw`border-t-zinc-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(244 244 245 / var(--tw-border-opacity))",
})
;"##### ; "702")]
#[test_case(r#####"tw`border-t-zinc-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(228 228 231 / var(--tw-border-opacity))",
})
;"##### ; "703")]
#[test_case(r#####"tw`border-t-zinc-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(212 212 216 / var(--tw-border-opacity))",
})
;"##### ; "704")]
#[test_case(r#####"tw`border-t-zinc-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(161 161 170 / var(--tw-border-opacity))",
})
;"##### ; "705")]
#[test_case(r#####"tw`border-t-zinc-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(113 113 122 / var(--tw-border-opacity))",
})
;"##### ; "706")]
#[test_case(r#####"tw`border-t-zinc-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(82 82 91 / var(--tw-border-opacity))",
})
;"##### ; "707")]
#[test_case(r#####"tw`border-t-zinc-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(63 63 70 / var(--tw-border-opacity))",
})
;"##### ; "708")]
#[test_case(r#####"tw`border-t-zinc-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(39 39 42 / var(--tw-border-opacity))",
})
;"##### ; "709")]
#[test_case(r#####"tw`border-t-zinc-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(24 24 27 / var(--tw-border-opacity))",
})
;"##### ; "710")]
#[test_case(r#####"tw`border-t-neutral-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "711")]
#[test_case(r#####"tw`border-t-neutral-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(245 245 245 / var(--tw-border-opacity))",
})
;"##### ; "712")]
#[test_case(r#####"tw`border-t-neutral-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(229 229 229 / var(--tw-border-opacity))",
})
;"##### ; "713")]
#[test_case(r#####"tw`border-t-neutral-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(212 212 212 / var(--tw-border-opacity))",
})
;"##### ; "714")]
#[test_case(r#####"tw`border-t-neutral-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(163 163 163 / var(--tw-border-opacity))",
})
;"##### ; "715")]
#[test_case(r#####"tw`border-t-neutral-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(115 115 115 / var(--tw-border-opacity))",
})
;"##### ; "716")]
#[test_case(r#####"tw`border-t-neutral-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(82 82 82 / var(--tw-border-opacity))",
})
;"##### ; "717")]
#[test_case(r#####"tw`border-t-neutral-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(64 64 64 / var(--tw-border-opacity))",
})
;"##### ; "718")]
#[test_case(r#####"tw`border-t-neutral-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(38 38 38 / var(--tw-border-opacity))",
})
;"##### ; "719")]
#[test_case(r#####"tw`border-t-neutral-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(23 23 23 / var(--tw-border-opacity))",
})
;"##### ; "720")]
#[test_case(r#####"tw`border-t-stone-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(250 250 249 / var(--tw-border-opacity))",
})
;"##### ; "721")]
#[test_case(r#####"tw`border-t-stone-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(245 245 244 / var(--tw-border-opacity))",
})
;"##### ; "722")]
#[test_case(r#####"tw`border-t-stone-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(231 229 228 / var(--tw-border-opacity))",
})
;"##### ; "723")]
#[test_case(r#####"tw`border-t-stone-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(214 211 209 / var(--tw-border-opacity))",
})
;"##### ; "724")]
#[test_case(r#####"tw`border-t-stone-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(168 162 158 / var(--tw-border-opacity))",
})
;"##### ; "725")]
#[test_case(r#####"tw`border-t-stone-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(120 113 108 / var(--tw-border-opacity))",
})
;"##### ; "726")]
#[test_case(r#####"tw`border-t-stone-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(87 83 78 / var(--tw-border-opacity))",
})
;"##### ; "727")]
#[test_case(r#####"tw`border-t-stone-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(68 64 60 / var(--tw-border-opacity))",
})
;"##### ; "728")]
#[test_case(r#####"tw`border-t-stone-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(41 37 36 / var(--tw-border-opacity))",
})
;"##### ; "729")]
#[test_case(r#####"tw`border-t-stone-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(28 25 23 / var(--tw-border-opacity))",
})
;"##### ; "730")]
#[test_case(r#####"tw`border-t-red-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 242 242 / var(--tw-border-opacity))",
})
;"##### ; "731")]
#[test_case(r#####"tw`border-t-red-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 226 226 / var(--tw-border-opacity))",
})
;"##### ; "732")]
#[test_case(r#####"tw`border-t-red-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 202 202 / var(--tw-border-opacity))",
})
;"##### ; "733")]
#[test_case(r#####"tw`border-t-red-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(252 165 165 / var(--tw-border-opacity))",
})
;"##### ; "734")]
#[test_case(r#####"tw`border-t-red-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(248 113 113 / var(--tw-border-opacity))",
})
;"##### ; "735")]
#[test_case(r#####"tw`border-t-red-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(239 68 68 / var(--tw-border-opacity))",
})
;"##### ; "736")]
#[test_case(r#####"tw`border-t-red-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(220 38 38 / var(--tw-border-opacity))",
})
;"##### ; "737")]
#[test_case(r#####"tw`border-t-red-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(185 28 28 / var(--tw-border-opacity))",
})
;"##### ; "738")]
#[test_case(r#####"tw`border-t-red-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(153 27 27 / var(--tw-border-opacity))",
})
;"##### ; "739")]
#[test_case(r#####"tw`border-t-red-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(127 29 29 / var(--tw-border-opacity))",
})
;"##### ; "740")]
#[test_case(r#####"tw`border-t-orange-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 247 237 / var(--tw-border-opacity))",
})
;"##### ; "741")]
#[test_case(r#####"tw`border-t-orange-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 237 213 / var(--tw-border-opacity))",
})
;"##### ; "742")]
#[test_case(r#####"tw`border-t-orange-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 215 170 / var(--tw-border-opacity))",
})
;"##### ; "743")]
#[test_case(r#####"tw`border-t-orange-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(253 186 116 / var(--tw-border-opacity))",
})
;"##### ; "744")]
#[test_case(r#####"tw`border-t-orange-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(251 146 60 / var(--tw-border-opacity))",
})
;"##### ; "745")]
#[test_case(r#####"tw`border-t-orange-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(249 115 22 / var(--tw-border-opacity))",
})
;"##### ; "746")]
#[test_case(r#####"tw`border-t-orange-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(234 88 12 / var(--tw-border-opacity))",
})
;"##### ; "747")]
#[test_case(r#####"tw`border-t-orange-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(194 65 12 / var(--tw-border-opacity))",
})
;"##### ; "748")]
#[test_case(r#####"tw`border-t-orange-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(154 52 18 / var(--tw-border-opacity))",
})
;"##### ; "749")]
#[test_case(r#####"tw`border-t-orange-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(124 45 18 / var(--tw-border-opacity))",
})
;"##### ; "750")]
#[test_case(r#####"tw`border-t-amber-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 251 235 / var(--tw-border-opacity))",
})
;"##### ; "751")]
#[test_case(r#####"tw`border-t-amber-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 243 199 / var(--tw-border-opacity))",
})
;"##### ; "752")]
#[test_case(r#####"tw`border-t-amber-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(253 230 138 / var(--tw-border-opacity))",
})
;"##### ; "753")]
#[test_case(r#####"tw`border-t-amber-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(252 211 77 / var(--tw-border-opacity))",
})
;"##### ; "754")]
#[test_case(r#####"tw`border-t-amber-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(251 191 36 / var(--tw-border-opacity))",
})
;"##### ; "755")]
#[test_case(r#####"tw`border-t-amber-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(245 158 11 / var(--tw-border-opacity))",
})
;"##### ; "756")]
#[test_case(r#####"tw`border-t-amber-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(217 119 6 / var(--tw-border-opacity))",
})
;"##### ; "757")]
#[test_case(r#####"tw`border-t-amber-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(180 83 9 / var(--tw-border-opacity))",
})
;"##### ; "758")]
#[test_case(r#####"tw`border-t-amber-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(146 64 14 / var(--tw-border-opacity))",
})
;"##### ; "759")]
#[test_case(r#####"tw`border-t-amber-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(120 53 15 / var(--tw-border-opacity))",
})
;"##### ; "760")]
#[test_case(r#####"tw`border-t-yellow-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 252 232 / var(--tw-border-opacity))",
})
;"##### ; "761")]
#[test_case(r#####"tw`border-t-yellow-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 249 195 / var(--tw-border-opacity))",
})
;"##### ; "762")]
#[test_case(r#####"tw`border-t-yellow-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 240 138 / var(--tw-border-opacity))",
})
;"##### ; "763")]
#[test_case(r#####"tw`border-t-yellow-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(253 224 71 / var(--tw-border-opacity))",
})
;"##### ; "764")]
#[test_case(r#####"tw`border-t-yellow-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(250 204 21 / var(--tw-border-opacity))",
})
;"##### ; "765")]
#[test_case(r#####"tw`border-t-yellow-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(234 179 8 / var(--tw-border-opacity))",
})
;"##### ; "766")]
#[test_case(r#####"tw`border-t-yellow-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(202 138 4 / var(--tw-border-opacity))",
})
;"##### ; "767")]
#[test_case(r#####"tw`border-t-yellow-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(161 98 7 / var(--tw-border-opacity))",
})
;"##### ; "768")]
#[test_case(r#####"tw`border-t-yellow-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(133 77 14 / var(--tw-border-opacity))",
})
;"##### ; "769")]
#[test_case(r#####"tw`border-t-yellow-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(113 63 18 / var(--tw-border-opacity))",
})
;"##### ; "770")]
#[test_case(r#####"tw`border-t-lime-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(247 254 231 / var(--tw-border-opacity))",
})
;"##### ; "771")]
#[test_case(r#####"tw`border-t-lime-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(236 252 203 / var(--tw-border-opacity))",
})
;"##### ; "772")]
#[test_case(r#####"tw`border-t-lime-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(217 249 157 / var(--tw-border-opacity))",
})
;"##### ; "773")]
#[test_case(r#####"tw`border-t-lime-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(190 242 100 / var(--tw-border-opacity))",
})
;"##### ; "774")]
#[test_case(r#####"tw`border-t-lime-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(163 230 53 / var(--tw-border-opacity))",
})
;"##### ; "775")]
#[test_case(r#####"tw`border-t-lime-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(132 204 22 / var(--tw-border-opacity))",
})
;"##### ; "776")]
#[test_case(r#####"tw`border-t-lime-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(101 163 13 / var(--tw-border-opacity))",
})
;"##### ; "777")]
#[test_case(r#####"tw`border-t-lime-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(77 124 15 / var(--tw-border-opacity))",
})
;"##### ; "778")]
#[test_case(r#####"tw`border-t-lime-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(63 98 18 / var(--tw-border-opacity))",
})
;"##### ; "779")]
#[test_case(r#####"tw`border-t-lime-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(54 83 20 / var(--tw-border-opacity))",
})
;"##### ; "780")]
#[test_case(r#####"tw`border-t-green-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(240 253 244 / var(--tw-border-opacity))",
})
;"##### ; "781")]
#[test_case(r#####"tw`border-t-green-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(220 252 231 / var(--tw-border-opacity))",
})
;"##### ; "782")]
#[test_case(r#####"tw`border-t-green-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(187 247 208 / var(--tw-border-opacity))",
})
;"##### ; "783")]
#[test_case(r#####"tw`border-t-green-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(134 239 172 / var(--tw-border-opacity))",
})
;"##### ; "784")]
#[test_case(r#####"tw`border-t-green-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(74 222 128 / var(--tw-border-opacity))",
})
;"##### ; "785")]
#[test_case(r#####"tw`border-t-green-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(34 197 94 / var(--tw-border-opacity))",
})
;"##### ; "786")]
#[test_case(r#####"tw`border-t-green-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(22 163 74 / var(--tw-border-opacity))",
})
;"##### ; "787")]
#[test_case(r#####"tw`border-t-green-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(21 128 61 / var(--tw-border-opacity))",
})
;"##### ; "788")]
#[test_case(r#####"tw`border-t-green-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(22 101 52 / var(--tw-border-opacity))",
})
;"##### ; "789")]
#[test_case(r#####"tw`border-t-green-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(20 83 45 / var(--tw-border-opacity))",
})
;"##### ; "790")]
#[test_case(r#####"tw`border-t-emerald-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(236 253 245 / var(--tw-border-opacity))",
})
;"##### ; "791")]
#[test_case(r#####"tw`border-t-emerald-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(209 250 229 / var(--tw-border-opacity))",
})
;"##### ; "792")]
#[test_case(r#####"tw`border-t-emerald-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(167 243 208 / var(--tw-border-opacity))",
})
;"##### ; "793")]
#[test_case(r#####"tw`border-t-emerald-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(110 231 183 / var(--tw-border-opacity))",
})
;"##### ; "794")]
#[test_case(r#####"tw`border-t-emerald-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(52 211 153 / var(--tw-border-opacity))",
})
;"##### ; "795")]
#[test_case(r#####"tw`border-t-emerald-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(16 185 129 / var(--tw-border-opacity))",
})
;"##### ; "796")]
#[test_case(r#####"tw`border-t-emerald-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(5 150 105 / var(--tw-border-opacity))",
})
;"##### ; "797")]
#[test_case(r#####"tw`border-t-emerald-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(4 120 87 / var(--tw-border-opacity))",
})
;"##### ; "798")]
#[test_case(r#####"tw`border-t-emerald-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(6 95 70 / var(--tw-border-opacity))",
})
;"##### ; "799")]
#[test_case(r#####"tw`border-t-emerald-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(6 78 59 / var(--tw-border-opacity))",
})
;"##### ; "800")]
#[test_case(r#####"tw`border-t-teal-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(240 253 250 / var(--tw-border-opacity))",
})
;"##### ; "801")]
#[test_case(r#####"tw`border-t-teal-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(204 251 241 / var(--tw-border-opacity))",
})
;"##### ; "802")]
#[test_case(r#####"tw`border-t-teal-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(153 246 228 / var(--tw-border-opacity))",
})
;"##### ; "803")]
#[test_case(r#####"tw`border-t-teal-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(94 234 212 / var(--tw-border-opacity))",
})
;"##### ; "804")]
#[test_case(r#####"tw`border-t-teal-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(45 212 191 / var(--tw-border-opacity))",
})
;"##### ; "805")]
#[test_case(r#####"tw`border-t-teal-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(20 184 166 / var(--tw-border-opacity))",
})
;"##### ; "806")]
#[test_case(r#####"tw`border-t-teal-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(13 148 136 / var(--tw-border-opacity))",
})
;"##### ; "807")]
#[test_case(r#####"tw`border-t-teal-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(15 118 110 / var(--tw-border-opacity))",
})
;"##### ; "808")]
#[test_case(r#####"tw`border-t-teal-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(17 94 89 / var(--tw-border-opacity))",
})
;"##### ; "809")]
#[test_case(r#####"tw`border-t-teal-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(19 78 74 / var(--tw-border-opacity))",
})
;"##### ; "810")]
#[test_case(r#####"tw`border-t-cyan-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(236 254 255 / var(--tw-border-opacity))",
})
;"##### ; "811")]
#[test_case(r#####"tw`border-t-cyan-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(207 250 254 / var(--tw-border-opacity))",
})
;"##### ; "812")]
#[test_case(r#####"tw`border-t-cyan-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(165 243 252 / var(--tw-border-opacity))",
})
;"##### ; "813")]
#[test_case(r#####"tw`border-t-cyan-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(103 232 249 / var(--tw-border-opacity))",
})
;"##### ; "814")]
#[test_case(r#####"tw`border-t-cyan-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(34 211 238 / var(--tw-border-opacity))",
})
;"##### ; "815")]
#[test_case(r#####"tw`border-t-cyan-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(6 182 212 / var(--tw-border-opacity))",
})
;"##### ; "816")]
#[test_case(r#####"tw`border-t-cyan-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(8 145 178 / var(--tw-border-opacity))",
})
;"##### ; "817")]
#[test_case(r#####"tw`border-t-cyan-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(14 116 144 / var(--tw-border-opacity))",
})
;"##### ; "818")]
#[test_case(r#####"tw`border-t-cyan-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(21 94 117 / var(--tw-border-opacity))",
})
;"##### ; "819")]
#[test_case(r#####"tw`border-t-cyan-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(22 78 99 / var(--tw-border-opacity))",
})
;"##### ; "820")]
#[test_case(r#####"tw`border-t-sky-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(240 249 255 / var(--tw-border-opacity))",
})
;"##### ; "821")]
#[test_case(r#####"tw`border-t-sky-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(224 242 254 / var(--tw-border-opacity))",
})
;"##### ; "822")]
#[test_case(r#####"tw`border-t-sky-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(186 230 253 / var(--tw-border-opacity))",
})
;"##### ; "823")]
#[test_case(r#####"tw`border-t-sky-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(125 211 252 / var(--tw-border-opacity))",
})
;"##### ; "824")]
#[test_case(r#####"tw`border-t-sky-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(56 189 248 / var(--tw-border-opacity))",
})
;"##### ; "825")]
#[test_case(r#####"tw`border-t-sky-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(14 165 233 / var(--tw-border-opacity))",
})
;"##### ; "826")]
#[test_case(r#####"tw`border-t-sky-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(2 132 199 / var(--tw-border-opacity))",
})
;"##### ; "827")]
#[test_case(r#####"tw`border-t-sky-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(3 105 161 / var(--tw-border-opacity))",
})
;"##### ; "828")]
#[test_case(r#####"tw`border-t-sky-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(7 89 133 / var(--tw-border-opacity))",
})
;"##### ; "829")]
#[test_case(r#####"tw`border-t-sky-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(12 74 110 / var(--tw-border-opacity))",
})
;"##### ; "830")]
#[test_case(r#####"tw`border-t-blue-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(239 246 255 / var(--tw-border-opacity))",
})
;"##### ; "831")]
#[test_case(r#####"tw`border-t-blue-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(219 234 254 / var(--tw-border-opacity))",
})
;"##### ; "832")]
#[test_case(r#####"tw`border-t-blue-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(191 219 254 / var(--tw-border-opacity))",
})
;"##### ; "833")]
#[test_case(r#####"tw`border-t-blue-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(147 197 253 / var(--tw-border-opacity))",
})
;"##### ; "834")]
#[test_case(r#####"tw`border-t-blue-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(96 165 250 / var(--tw-border-opacity))",
})
;"##### ; "835")]
#[test_case(r#####"tw`border-t-blue-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(59 130 246 / var(--tw-border-opacity))",
})
;"##### ; "836")]
#[test_case(r#####"tw`border-t-blue-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(37 99 235 / var(--tw-border-opacity))",
})
;"##### ; "837")]
#[test_case(r#####"tw`border-t-blue-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(29 78 216 / var(--tw-border-opacity))",
})
;"##### ; "838")]
#[test_case(r#####"tw`border-t-blue-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(30 64 175 / var(--tw-border-opacity))",
})
;"##### ; "839")]
#[test_case(r#####"tw`border-t-blue-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(30 58 138 / var(--tw-border-opacity))",
})
;"##### ; "840")]
#[test_case(r#####"tw`border-t-indigo-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(238 242 255 / var(--tw-border-opacity))",
})
;"##### ; "841")]
#[test_case(r#####"tw`border-t-indigo-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(224 231 255 / var(--tw-border-opacity))",
})
;"##### ; "842")]
#[test_case(r#####"tw`border-t-indigo-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(199 210 254 / var(--tw-border-opacity))",
})
;"##### ; "843")]
#[test_case(r#####"tw`border-t-indigo-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(165 180 252 / var(--tw-border-opacity))",
})
;"##### ; "844")]
#[test_case(r#####"tw`border-t-indigo-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(129 140 248 / var(--tw-border-opacity))",
})
;"##### ; "845")]
#[test_case(r#####"tw`border-t-indigo-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(99 102 241 / var(--tw-border-opacity))",
})
;"##### ; "846")]
#[test_case(r#####"tw`border-t-indigo-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(79 70 229 / var(--tw-border-opacity))",
})
;"##### ; "847")]
#[test_case(r#####"tw`border-t-indigo-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(67 56 202 / var(--tw-border-opacity))",
})
;"##### ; "848")]
#[test_case(r#####"tw`border-t-indigo-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(55 48 163 / var(--tw-border-opacity))",
})
;"##### ; "849")]
#[test_case(r#####"tw`border-t-indigo-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(49 46 129 / var(--tw-border-opacity))",
})
;"##### ; "850")]
#[test_case(r#####"tw`border-t-violet-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(245 243 255 / var(--tw-border-opacity))",
})
;"##### ; "851")]
#[test_case(r#####"tw`border-t-violet-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(237 233 254 / var(--tw-border-opacity))",
})
;"##### ; "852")]
#[test_case(r#####"tw`border-t-violet-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(221 214 254 / var(--tw-border-opacity))",
})
;"##### ; "853")]
#[test_case(r#####"tw`border-t-violet-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(196 181 253 / var(--tw-border-opacity))",
})
;"##### ; "854")]
#[test_case(r#####"tw`border-t-violet-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(167 139 250 / var(--tw-border-opacity))",
})
;"##### ; "855")]
#[test_case(r#####"tw`border-t-violet-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(139 92 246 / var(--tw-border-opacity))",
})
;"##### ; "856")]
#[test_case(r#####"tw`border-t-violet-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(124 58 237 / var(--tw-border-opacity))",
})
;"##### ; "857")]
#[test_case(r#####"tw`border-t-violet-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(109 40 217 / var(--tw-border-opacity))",
})
;"##### ; "858")]
#[test_case(r#####"tw`border-t-violet-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(91 33 182 / var(--tw-border-opacity))",
})
;"##### ; "859")]
#[test_case(r#####"tw`border-t-violet-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(76 29 149 / var(--tw-border-opacity))",
})
;"##### ; "860")]
#[test_case(r#####"tw`border-t-purple-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(250 245 255 / var(--tw-border-opacity))",
})
;"##### ; "861")]
#[test_case(r#####"tw`border-t-purple-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(243 232 255 / var(--tw-border-opacity))",
})
;"##### ; "862")]
#[test_case(r#####"tw`border-t-purple-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(233 213 255 / var(--tw-border-opacity))",
})
;"##### ; "863")]
#[test_case(r#####"tw`border-t-purple-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(216 180 254 / var(--tw-border-opacity))",
})
;"##### ; "864")]
#[test_case(r#####"tw`border-t-purple-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(192 132 252 / var(--tw-border-opacity))",
})
;"##### ; "865")]
#[test_case(r#####"tw`border-t-purple-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(168 85 247 / var(--tw-border-opacity))",
})
;"##### ; "866")]
#[test_case(r#####"tw`border-t-purple-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(147 51 234 / var(--tw-border-opacity))",
})
;"##### ; "867")]
#[test_case(r#####"tw`border-t-purple-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(126 34 206 / var(--tw-border-opacity))",
})
;"##### ; "868")]
#[test_case(r#####"tw`border-t-purple-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(107 33 168 / var(--tw-border-opacity))",
})
;"##### ; "869")]
#[test_case(r#####"tw`border-t-purple-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(88 28 135 / var(--tw-border-opacity))",
})
;"##### ; "870")]
#[test_case(r#####"tw`border-t-fuchsia-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(253 244 255 / var(--tw-border-opacity))",
})
;"##### ; "871")]
#[test_case(r#####"tw`border-t-fuchsia-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(250 232 255 / var(--tw-border-opacity))",
})
;"##### ; "872")]
#[test_case(r#####"tw`border-t-fuchsia-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(245 208 254 / var(--tw-border-opacity))",
})
;"##### ; "873")]
#[test_case(r#####"tw`border-t-fuchsia-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(240 171 252 / var(--tw-border-opacity))",
})
;"##### ; "874")]
#[test_case(r#####"tw`border-t-fuchsia-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(232 121 249 / var(--tw-border-opacity))",
})
;"##### ; "875")]
#[test_case(r#####"tw`border-t-fuchsia-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(217 70 239 / var(--tw-border-opacity))",
})
;"##### ; "876")]
#[test_case(r#####"tw`border-t-fuchsia-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(192 38 211 / var(--tw-border-opacity))",
})
;"##### ; "877")]
#[test_case(r#####"tw`border-t-fuchsia-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(162 28 175 / var(--tw-border-opacity))",
})
;"##### ; "878")]
#[test_case(r#####"tw`border-t-fuchsia-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(134 25 143 / var(--tw-border-opacity))",
})
;"##### ; "879")]
#[test_case(r#####"tw`border-t-fuchsia-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(112 26 117 / var(--tw-border-opacity))",
})
;"##### ; "880")]
#[test_case(r#####"tw`border-t-pink-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(253 242 248 / var(--tw-border-opacity))",
})
;"##### ; "881")]
#[test_case(r#####"tw`border-t-pink-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(252 231 243 / var(--tw-border-opacity))",
})
;"##### ; "882")]
#[test_case(r#####"tw`border-t-pink-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(251 207 232 / var(--tw-border-opacity))",
})
;"##### ; "883")]
#[test_case(r#####"tw`border-t-pink-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(249 168 212 / var(--tw-border-opacity))",
})
;"##### ; "884")]
#[test_case(r#####"tw`border-t-pink-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(244 114 182 / var(--tw-border-opacity))",
})
;"##### ; "885")]
#[test_case(r#####"tw`border-t-pink-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(236 72 153 / var(--tw-border-opacity))",
})
;"##### ; "886")]
#[test_case(r#####"tw`border-t-pink-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(219 39 119 / var(--tw-border-opacity))",
})
;"##### ; "887")]
#[test_case(r#####"tw`border-t-pink-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(190 24 93 / var(--tw-border-opacity))",
})
;"##### ; "888")]
#[test_case(r#####"tw`border-t-pink-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(157 23 77 / var(--tw-border-opacity))",
})
;"##### ; "889")]
#[test_case(r#####"tw`border-t-pink-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(131 24 67 / var(--tw-border-opacity))",
})
;"##### ; "890")]
#[test_case(r#####"tw`border-t-rose-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 241 242 / var(--tw-border-opacity))",
})
;"##### ; "891")]
#[test_case(r#####"tw`border-t-rose-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 228 230 / var(--tw-border-opacity))",
})
;"##### ; "892")]
#[test_case(r#####"tw`border-t-rose-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(254 205 211 / var(--tw-border-opacity))",
})
;"##### ; "893")]
#[test_case(r#####"tw`border-t-rose-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(253 164 175 / var(--tw-border-opacity))",
})
;"##### ; "894")]
#[test_case(r#####"tw`border-t-rose-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(251 113 133 / var(--tw-border-opacity))",
})
;"##### ; "895")]
#[test_case(r#####"tw`border-t-rose-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(244 63 94 / var(--tw-border-opacity))",
})
;"##### ; "896")]
#[test_case(r#####"tw`border-t-rose-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(225 29 72 / var(--tw-border-opacity))",
})
;"##### ; "897")]
#[test_case(r#####"tw`border-t-rose-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(190 18 60 / var(--tw-border-opacity))",
})
;"##### ; "898")]
#[test_case(r#####"tw`border-t-rose-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(159 18 57 / var(--tw-border-opacity))",
})
;"##### ; "899")]
#[test_case(r#####"tw`border-t-rose-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(136 19 55 / var(--tw-border-opacity))",
})
;"##### ; "900")]
#[test_case(r#####"tw`border-r-inherit
border-r-current
border-r-transparent
border-r-black
border-r-white
border-r-slate-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(255 255 255 / var(--tw-border-opacity))",
})
;"##### ; "901")]
#[test_case(r#####"tw`border-r-slate-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(241 245 249 / var(--tw-border-opacity))",
})
;"##### ; "902")]
#[test_case(r#####"tw`border-r-slate-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(226 232 240 / var(--tw-border-opacity))",
})
;"##### ; "903")]
#[test_case(r#####"tw`border-r-slate-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(203 213 225 / var(--tw-border-opacity))",
})
;"##### ; "904")]
#[test_case(r#####"tw`border-r-slate-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(148 163 184 / var(--tw-border-opacity))",
})
;"##### ; "905")]
#[test_case(r#####"tw`border-r-slate-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(100 116 139 / var(--tw-border-opacity))",
})
;"##### ; "906")]
#[test_case(r#####"tw`border-r-slate-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(71 85 105 / var(--tw-border-opacity))",
})
;"##### ; "907")]
#[test_case(r#####"tw`border-r-slate-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(51 65 85 / var(--tw-border-opacity))",
})
;"##### ; "908")]
#[test_case(r#####"tw`border-r-slate-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(30 41 59 / var(--tw-border-opacity))",
})
;"##### ; "909")]
#[test_case(r#####"tw`border-r-slate-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(15 23 42 / var(--tw-border-opacity))",
})
;"##### ; "910")]
#[test_case(r#####"tw`border-r-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "911")]
#[test_case(r#####"tw`border-r-gray-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(243 244 246 / var(--tw-border-opacity))",
})
;"##### ; "912")]
#[test_case(r#####"tw`border-r-gray-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(229 231 235 / var(--tw-border-opacity))",
})
;"##### ; "913")]
#[test_case(r#####"tw`border-r-gray-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(209 213 219 / var(--tw-border-opacity))",
})
;"##### ; "914")]
#[test_case(r#####"tw`border-r-gray-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(156 163 175 / var(--tw-border-opacity))",
})
;"##### ; "915")]
#[test_case(r#####"tw`border-r-gray-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(107 114 128 / var(--tw-border-opacity))",
})
;"##### ; "916")]
#[test_case(r#####"tw`border-r-gray-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(75 85 99 / var(--tw-border-opacity))",
})
;"##### ; "917")]
#[test_case(r#####"tw`border-r-gray-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(55 65 81 / var(--tw-border-opacity))",
})
;"##### ; "918")]
#[test_case(r#####"tw`border-r-gray-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(31 41 55 / var(--tw-border-opacity))",
})
;"##### ; "919")]
#[test_case(r#####"tw`border-r-gray-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(17 24 39 / var(--tw-border-opacity))",
})
;"##### ; "920")]
#[test_case(r#####"tw`border-r-zinc-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "921")]
#[test_case(r#####"tw`border-r-zinc-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(244 244 245 / var(--tw-border-opacity))",
})
;"##### ; "922")]
#[test_case(r#####"tw`border-r-zinc-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(228 228 231 / var(--tw-border-opacity))",
})
;"##### ; "923")]
#[test_case(r#####"tw`border-r-zinc-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(212 212 216 / var(--tw-border-opacity))",
})
;"##### ; "924")]
#[test_case(r#####"tw`border-r-zinc-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(161 161 170 / var(--tw-border-opacity))",
})
;"##### ; "925")]
#[test_case(r#####"tw`border-r-zinc-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(113 113 122 / var(--tw-border-opacity))",
})
;"##### ; "926")]
#[test_case(r#####"tw`border-r-zinc-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(82 82 91 / var(--tw-border-opacity))",
})
;"##### ; "927")]
#[test_case(r#####"tw`border-r-zinc-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(63 63 70 / var(--tw-border-opacity))",
})
;"##### ; "928")]
#[test_case(r#####"tw`border-r-zinc-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(39 39 42 / var(--tw-border-opacity))",
})
;"##### ; "929")]
#[test_case(r#####"tw`border-r-zinc-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(24 24 27 / var(--tw-border-opacity))",
})
;"##### ; "930")]
#[test_case(r#####"tw`border-r-neutral-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "931")]
#[test_case(r#####"tw`border-r-neutral-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(245 245 245 / var(--tw-border-opacity))",
})
;"##### ; "932")]
#[test_case(r#####"tw`border-r-neutral-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(229 229 229 / var(--tw-border-opacity))",
})
;"##### ; "933")]
#[test_case(r#####"tw`border-r-neutral-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(212 212 212 / var(--tw-border-opacity))",
})
;"##### ; "934")]
#[test_case(r#####"tw`border-r-neutral-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(163 163 163 / var(--tw-border-opacity))",
})
;"##### ; "935")]
#[test_case(r#####"tw`border-r-neutral-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(115 115 115 / var(--tw-border-opacity))",
})
;"##### ; "936")]
#[test_case(r#####"tw`border-r-neutral-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(82 82 82 / var(--tw-border-opacity))",
})
;"##### ; "937")]
#[test_case(r#####"tw`border-r-neutral-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(64 64 64 / var(--tw-border-opacity))",
})
;"##### ; "938")]
#[test_case(r#####"tw`border-r-neutral-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(38 38 38 / var(--tw-border-opacity))",
})
;"##### ; "939")]
#[test_case(r#####"tw`border-r-neutral-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(23 23 23 / var(--tw-border-opacity))",
})
;"##### ; "940")]
#[test_case(r#####"tw`border-r-stone-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(250 250 249 / var(--tw-border-opacity))",
})
;"##### ; "941")]
#[test_case(r#####"tw`border-r-stone-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(245 245 244 / var(--tw-border-opacity))",
})
;"##### ; "942")]
#[test_case(r#####"tw`border-r-stone-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(231 229 228 / var(--tw-border-opacity))",
})
;"##### ; "943")]
#[test_case(r#####"tw`border-r-stone-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(214 211 209 / var(--tw-border-opacity))",
})
;"##### ; "944")]
#[test_case(r#####"tw`border-r-stone-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(168 162 158 / var(--tw-border-opacity))",
})
;"##### ; "945")]
#[test_case(r#####"tw`border-r-stone-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(120 113 108 / var(--tw-border-opacity))",
})
;"##### ; "946")]
#[test_case(r#####"tw`border-r-stone-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(87 83 78 / var(--tw-border-opacity))",
})
;"##### ; "947")]
#[test_case(r#####"tw`border-r-stone-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(68 64 60 / var(--tw-border-opacity))",
})
;"##### ; "948")]
#[test_case(r#####"tw`border-r-stone-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(41 37 36 / var(--tw-border-opacity))",
})
;"##### ; "949")]
#[test_case(r#####"tw`border-r-stone-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(28 25 23 / var(--tw-border-opacity))",
})
;"##### ; "950")]
#[test_case(r#####"tw`border-r-red-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(254 242 242 / var(--tw-border-opacity))",
})
;"##### ; "951")]
#[test_case(r#####"tw`border-r-red-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(254 226 226 / var(--tw-border-opacity))",
})
;"##### ; "952")]
#[test_case(r#####"tw`border-r-red-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(254 202 202 / var(--tw-border-opacity))",
})
;"##### ; "953")]
#[test_case(r#####"tw`border-r-red-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(252 165 165 / var(--tw-border-opacity))",
})
;"##### ; "954")]
#[test_case(r#####"tw`border-r-red-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(248 113 113 / var(--tw-border-opacity))",
})
;"##### ; "955")]
#[test_case(r#####"tw`border-r-red-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(239 68 68 / var(--tw-border-opacity))",
})
;"##### ; "956")]
#[test_case(r#####"tw`border-r-red-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(220 38 38 / var(--tw-border-opacity))",
})
;"##### ; "957")]
#[test_case(r#####"tw`border-r-red-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(185 28 28 / var(--tw-border-opacity))",
})
;"##### ; "958")]
#[test_case(r#####"tw`border-r-red-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(153 27 27 / var(--tw-border-opacity))",
})
;"##### ; "959")]
#[test_case(r#####"tw`border-r-red-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(127 29 29 / var(--tw-border-opacity))",
})
;"##### ; "960")]
#[test_case(r#####"tw`border-r-orange-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(255 247 237 / var(--tw-border-opacity))",
})
;"##### ; "961")]
#[test_case(r#####"tw`border-r-orange-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(255 237 213 / var(--tw-border-opacity))",
})
;"##### ; "962")]
#[test_case(r#####"tw`border-r-orange-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(254 215 170 / var(--tw-border-opacity))",
})
;"##### ; "963")]
#[test_case(r#####"tw`border-r-orange-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(253 186 116 / var(--tw-border-opacity))",
})
;"##### ; "964")]
#[test_case(r#####"tw`border-r-orange-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(251 146 60 / var(--tw-border-opacity))",
})
;"##### ; "965")]
#[test_case(r#####"tw`border-r-orange-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(249 115 22 / var(--tw-border-opacity))",
})
;"##### ; "966")]
#[test_case(r#####"tw`border-r-orange-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(234 88 12 / var(--tw-border-opacity))",
})
;"##### ; "967")]
#[test_case(r#####"tw`border-r-orange-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(194 65 12 / var(--tw-border-opacity))",
})
;"##### ; "968")]
#[test_case(r#####"tw`border-r-orange-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(154 52 18 / var(--tw-border-opacity))",
})
;"##### ; "969")]
#[test_case(r#####"tw`border-r-orange-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(124 45 18 / var(--tw-border-opacity))",
})
;"##### ; "970")]
#[test_case(r#####"tw`border-r-amber-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(255 251 235 / var(--tw-border-opacity))",
})
;"##### ; "971")]
#[test_case(r#####"tw`border-r-amber-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(254 243 199 / var(--tw-border-opacity))",
})
;"##### ; "972")]
#[test_case(r#####"tw`border-r-amber-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(253 230 138 / var(--tw-border-opacity))",
})
;"##### ; "973")]
#[test_case(r#####"tw`border-r-amber-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(252 211 77 / var(--tw-border-opacity))",
})
;"##### ; "974")]
#[test_case(r#####"tw`border-r-amber-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(251 191 36 / var(--tw-border-opacity))",
})
;"##### ; "975")]
#[test_case(r#####"tw`border-r-amber-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(245 158 11 / var(--tw-border-opacity))",
})
;"##### ; "976")]
#[test_case(r#####"tw`border-r-amber-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(217 119 6 / var(--tw-border-opacity))",
})
;"##### ; "977")]
#[test_case(r#####"tw`border-r-amber-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(180 83 9 / var(--tw-border-opacity))",
})
;"##### ; "978")]
#[test_case(r#####"tw`border-r-amber-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(146 64 14 / var(--tw-border-opacity))",
})
;"##### ; "979")]
#[test_case(r#####"tw`border-r-amber-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(120 53 15 / var(--tw-border-opacity))",
})
;"##### ; "980")]
#[test_case(r#####"tw`border-r-yellow-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(254 252 232 / var(--tw-border-opacity))",
})
;"##### ; "981")]
#[test_case(r#####"tw`border-r-yellow-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(254 249 195 / var(--tw-border-opacity))",
})
;"##### ; "982")]
#[test_case(r#####"tw`border-r-yellow-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(254 240 138 / var(--tw-border-opacity))",
})
;"##### ; "983")]
#[test_case(r#####"tw`border-r-yellow-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(253 224 71 / var(--tw-border-opacity))",
})
;"##### ; "984")]
#[test_case(r#####"tw`border-r-yellow-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(250 204 21 / var(--tw-border-opacity))",
})
;"##### ; "985")]
#[test_case(r#####"tw`border-r-yellow-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(234 179 8 / var(--tw-border-opacity))",
})
;"##### ; "986")]
#[test_case(r#####"tw`border-r-yellow-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(202 138 4 / var(--tw-border-opacity))",
})
;"##### ; "987")]
#[test_case(r#####"tw`border-r-yellow-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(161 98 7 / var(--tw-border-opacity))",
})
;"##### ; "988")]
#[test_case(r#####"tw`border-r-yellow-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(133 77 14 / var(--tw-border-opacity))",
})
;"##### ; "989")]
#[test_case(r#####"tw`border-r-yellow-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(113 63 18 / var(--tw-border-opacity))",
})
;"##### ; "990")]
#[test_case(r#####"tw`border-r-lime-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(247 254 231 / var(--tw-border-opacity))",
})
;"##### ; "991")]
#[test_case(r#####"tw`border-r-lime-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(236 252 203 / var(--tw-border-opacity))",
})
;"##### ; "992")]
#[test_case(r#####"tw`border-r-lime-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(217 249 157 / var(--tw-border-opacity))",
})
;"##### ; "993")]
#[test_case(r#####"tw`border-r-lime-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(190 242 100 / var(--tw-border-opacity))",
})
;"##### ; "994")]
#[test_case(r#####"tw`border-r-lime-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(163 230 53 / var(--tw-border-opacity))",
})
;"##### ; "995")]
#[test_case(r#####"tw`border-r-lime-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(132 204 22 / var(--tw-border-opacity))",
})
;"##### ; "996")]
#[test_case(r#####"tw`border-r-lime-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(101 163 13 / var(--tw-border-opacity))",
})
;"##### ; "997")]
#[test_case(r#####"tw`border-r-lime-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(77 124 15 / var(--tw-border-opacity))",
})
;"##### ; "998")]
#[test_case(r#####"tw`border-r-lime-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(63 98 18 / var(--tw-border-opacity))",
})
;"##### ; "999")]
#[test_case(r#####"tw`border-r-lime-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(54 83 20 / var(--tw-border-opacity))",
})
;"##### ; "1000")]
#[test_case(r#####"tw`border-r-green-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(240 253 244 / var(--tw-border-opacity))",
})
;"##### ; "1001")]
#[test_case(r#####"tw`border-r-green-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(220 252 231 / var(--tw-border-opacity))",
})
;"##### ; "1002")]
#[test_case(r#####"tw`border-r-green-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(187 247 208 / var(--tw-border-opacity))",
})
;"##### ; "1003")]
#[test_case(r#####"tw`border-r-green-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(134 239 172 / var(--tw-border-opacity))",
})
;"##### ; "1004")]
#[test_case(r#####"tw`border-r-green-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(74 222 128 / var(--tw-border-opacity))",
})
;"##### ; "1005")]
#[test_case(r#####"tw`border-r-green-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(34 197 94 / var(--tw-border-opacity))",
})
;"##### ; "1006")]
#[test_case(r#####"tw`border-r-green-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(22 163 74 / var(--tw-border-opacity))",
})
;"##### ; "1007")]
#[test_case(r#####"tw`border-r-green-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(21 128 61 / var(--tw-border-opacity))",
})
;"##### ; "1008")]
#[test_case(r#####"tw`border-r-green-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(22 101 52 / var(--tw-border-opacity))",
})
;"##### ; "1009")]
#[test_case(r#####"tw`border-r-green-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(20 83 45 / var(--tw-border-opacity))",
})
;"##### ; "1010")]
#[test_case(r#####"tw`border-r-emerald-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(236 253 245 / var(--tw-border-opacity))",
})
;"##### ; "1011")]
#[test_case(r#####"tw`border-r-emerald-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(209 250 229 / var(--tw-border-opacity))",
})
;"##### ; "1012")]
#[test_case(r#####"tw`border-r-emerald-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(167 243 208 / var(--tw-border-opacity))",
})
;"##### ; "1013")]
#[test_case(r#####"tw`border-r-emerald-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(110 231 183 / var(--tw-border-opacity))",
})
;"##### ; "1014")]
#[test_case(r#####"tw`border-r-emerald-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(52 211 153 / var(--tw-border-opacity))",
})
;"##### ; "1015")]
#[test_case(r#####"tw`border-r-emerald-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(16 185 129 / var(--tw-border-opacity))",
})
;"##### ; "1016")]
#[test_case(r#####"tw`border-r-emerald-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(5 150 105 / var(--tw-border-opacity))",
})
;"##### ; "1017")]
#[test_case(r#####"tw`border-r-emerald-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(4 120 87 / var(--tw-border-opacity))",
})
;"##### ; "1018")]
#[test_case(r#####"tw`border-r-emerald-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(6 95 70 / var(--tw-border-opacity))",
})
;"##### ; "1019")]
#[test_case(r#####"tw`border-r-emerald-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(6 78 59 / var(--tw-border-opacity))",
})
;"##### ; "1020")]
#[test_case(r#####"tw`border-r-teal-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(240 253 250 / var(--tw-border-opacity))",
})
;"##### ; "1021")]
#[test_case(r#####"tw`border-r-teal-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(204 251 241 / var(--tw-border-opacity))",
})
;"##### ; "1022")]
#[test_case(r#####"tw`border-r-teal-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(153 246 228 / var(--tw-border-opacity))",
})
;"##### ; "1023")]
#[test_case(r#####"tw`border-r-teal-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(94 234 212 / var(--tw-border-opacity))",
})
;"##### ; "1024")]
#[test_case(r#####"tw`border-r-teal-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(45 212 191 / var(--tw-border-opacity))",
})
;"##### ; "1025")]
#[test_case(r#####"tw`border-r-teal-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(20 184 166 / var(--tw-border-opacity))",
})
;"##### ; "1026")]
#[test_case(r#####"tw`border-r-teal-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(13 148 136 / var(--tw-border-opacity))",
})
;"##### ; "1027")]
#[test_case(r#####"tw`border-r-teal-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(15 118 110 / var(--tw-border-opacity))",
})
;"##### ; "1028")]
#[test_case(r#####"tw`border-r-teal-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(17 94 89 / var(--tw-border-opacity))",
})
;"##### ; "1029")]
#[test_case(r#####"tw`border-r-teal-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(19 78 74 / var(--tw-border-opacity))",
})
;"##### ; "1030")]
#[test_case(r#####"tw`border-r-cyan-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(236 254 255 / var(--tw-border-opacity))",
})
;"##### ; "1031")]
#[test_case(r#####"tw`border-r-cyan-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(207 250 254 / var(--tw-border-opacity))",
})
;"##### ; "1032")]
#[test_case(r#####"tw`border-r-cyan-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(165 243 252 / var(--tw-border-opacity))",
})
;"##### ; "1033")]
#[test_case(r#####"tw`border-r-cyan-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(103 232 249 / var(--tw-border-opacity))",
})
;"##### ; "1034")]
#[test_case(r#####"tw`border-r-cyan-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(34 211 238 / var(--tw-border-opacity))",
})
;"##### ; "1035")]
#[test_case(r#####"tw`border-r-cyan-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(6 182 212 / var(--tw-border-opacity))",
})
;"##### ; "1036")]
#[test_case(r#####"tw`border-r-cyan-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(8 145 178 / var(--tw-border-opacity))",
})
;"##### ; "1037")]
#[test_case(r#####"tw`border-r-cyan-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(14 116 144 / var(--tw-border-opacity))",
})
;"##### ; "1038")]
#[test_case(r#####"tw`border-r-cyan-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(21 94 117 / var(--tw-border-opacity))",
})
;"##### ; "1039")]
#[test_case(r#####"tw`border-r-cyan-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(22 78 99 / var(--tw-border-opacity))",
})
;"##### ; "1040")]
#[test_case(r#####"tw`border-r-sky-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(240 249 255 / var(--tw-border-opacity))",
})
;"##### ; "1041")]
#[test_case(r#####"tw`border-r-sky-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(224 242 254 / var(--tw-border-opacity))",
})
;"##### ; "1042")]
#[test_case(r#####"tw`border-r-sky-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(186 230 253 / var(--tw-border-opacity))",
})
;"##### ; "1043")]
#[test_case(r#####"tw`border-r-sky-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(125 211 252 / var(--tw-border-opacity))",
})
;"##### ; "1044")]
#[test_case(r#####"tw`border-r-sky-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(56 189 248 / var(--tw-border-opacity))",
})
;"##### ; "1045")]
#[test_case(r#####"tw`border-r-sky-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(14 165 233 / var(--tw-border-opacity))",
})
;"##### ; "1046")]
#[test_case(r#####"tw`border-r-sky-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(2 132 199 / var(--tw-border-opacity))",
})
;"##### ; "1047")]
#[test_case(r#####"tw`border-r-sky-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(3 105 161 / var(--tw-border-opacity))",
})
;"##### ; "1048")]
#[test_case(r#####"tw`border-r-sky-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(7 89 133 / var(--tw-border-opacity))",
})
;"##### ; "1049")]
#[test_case(r#####"tw`border-r-sky-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(12 74 110 / var(--tw-border-opacity))",
})
;"##### ; "1050")]
#[test_case(r#####"tw`border-r-blue-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(239 246 255 / var(--tw-border-opacity))",
})
;"##### ; "1051")]
#[test_case(r#####"tw`border-r-blue-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(219 234 254 / var(--tw-border-opacity))",
})
;"##### ; "1052")]
#[test_case(r#####"tw`border-r-blue-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(191 219 254 / var(--tw-border-opacity))",
})
;"##### ; "1053")]
#[test_case(r#####"tw`border-r-blue-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(147 197 253 / var(--tw-border-opacity))",
})
;"##### ; "1054")]
#[test_case(r#####"tw`border-r-blue-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(96 165 250 / var(--tw-border-opacity))",
})
;"##### ; "1055")]
#[test_case(r#####"tw`border-r-blue-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(59 130 246 / var(--tw-border-opacity))",
})
;"##### ; "1056")]
#[test_case(r#####"tw`border-r-blue-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(37 99 235 / var(--tw-border-opacity))",
})
;"##### ; "1057")]
#[test_case(r#####"tw`border-r-blue-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(29 78 216 / var(--tw-border-opacity))",
})
;"##### ; "1058")]
#[test_case(r#####"tw`border-r-blue-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(30 64 175 / var(--tw-border-opacity))",
})
;"##### ; "1059")]
#[test_case(r#####"tw`border-r-blue-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(30 58 138 / var(--tw-border-opacity))",
})
;"##### ; "1060")]
#[test_case(r#####"tw`border-r-indigo-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(238 242 255 / var(--tw-border-opacity))",
})
;"##### ; "1061")]
#[test_case(r#####"tw`border-r-indigo-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(224 231 255 / var(--tw-border-opacity))",
})
;"##### ; "1062")]
#[test_case(r#####"tw`border-r-indigo-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(199 210 254 / var(--tw-border-opacity))",
})
;"##### ; "1063")]
#[test_case(r#####"tw`border-r-indigo-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(165 180 252 / var(--tw-border-opacity))",
})
;"##### ; "1064")]
#[test_case(r#####"tw`border-r-indigo-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(129 140 248 / var(--tw-border-opacity))",
})
;"##### ; "1065")]
#[test_case(r#####"tw`border-r-indigo-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(99 102 241 / var(--tw-border-opacity))",
})
;"##### ; "1066")]
#[test_case(r#####"tw`border-r-indigo-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(79 70 229 / var(--tw-border-opacity))",
})
;"##### ; "1067")]
#[test_case(r#####"tw`border-r-indigo-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(67 56 202 / var(--tw-border-opacity))",
})
;"##### ; "1068")]
#[test_case(r#####"tw`border-r-indigo-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(55 48 163 / var(--tw-border-opacity))",
})
;"##### ; "1069")]
#[test_case(r#####"tw`border-r-indigo-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(49 46 129 / var(--tw-border-opacity))",
})
;"##### ; "1070")]
#[test_case(r#####"tw`border-r-violet-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(245 243 255 / var(--tw-border-opacity))",
})
;"##### ; "1071")]
#[test_case(r#####"tw`border-r-violet-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(237 233 254 / var(--tw-border-opacity))",
})
;"##### ; "1072")]
#[test_case(r#####"tw`border-r-violet-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(221 214 254 / var(--tw-border-opacity))",
})
;"##### ; "1073")]
#[test_case(r#####"tw`border-r-violet-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(196 181 253 / var(--tw-border-opacity))",
})
;"##### ; "1074")]
#[test_case(r#####"tw`border-r-violet-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(167 139 250 / var(--tw-border-opacity))",
})
;"##### ; "1075")]
#[test_case(r#####"tw`border-r-violet-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(139 92 246 / var(--tw-border-opacity))",
})
;"##### ; "1076")]
#[test_case(r#####"tw`border-r-violet-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(124 58 237 / var(--tw-border-opacity))",
})
;"##### ; "1077")]
#[test_case(r#####"tw`border-r-violet-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(109 40 217 / var(--tw-border-opacity))",
})
;"##### ; "1078")]
#[test_case(r#####"tw`border-r-violet-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(91 33 182 / var(--tw-border-opacity))",
})
;"##### ; "1079")]
#[test_case(r#####"tw`border-r-violet-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(76 29 149 / var(--tw-border-opacity))",
})
;"##### ; "1080")]
#[test_case(r#####"tw`border-r-purple-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(250 245 255 / var(--tw-border-opacity))",
})
;"##### ; "1081")]
#[test_case(r#####"tw`border-r-purple-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(243 232 255 / var(--tw-border-opacity))",
})
;"##### ; "1082")]
#[test_case(r#####"tw`border-r-purple-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(233 213 255 / var(--tw-border-opacity))",
})
;"##### ; "1083")]
#[test_case(r#####"tw`border-r-purple-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(216 180 254 / var(--tw-border-opacity))",
})
;"##### ; "1084")]
#[test_case(r#####"tw`border-r-purple-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(192 132 252 / var(--tw-border-opacity))",
})
;"##### ; "1085")]
#[test_case(r#####"tw`border-r-purple-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(168 85 247 / var(--tw-border-opacity))",
})
;"##### ; "1086")]
#[test_case(r#####"tw`border-r-purple-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(147 51 234 / var(--tw-border-opacity))",
})
;"##### ; "1087")]
#[test_case(r#####"tw`border-r-purple-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(126 34 206 / var(--tw-border-opacity))",
})
;"##### ; "1088")]
#[test_case(r#####"tw`border-r-purple-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(107 33 168 / var(--tw-border-opacity))",
})
;"##### ; "1089")]
#[test_case(r#####"tw`border-r-purple-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(88 28 135 / var(--tw-border-opacity))",
})
;"##### ; "1090")]
#[test_case(r#####"tw`border-r-fuchsia-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(253 244 255 / var(--tw-border-opacity))",
})
;"##### ; "1091")]
#[test_case(r#####"tw`border-r-fuchsia-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(250 232 255 / var(--tw-border-opacity))",
})
;"##### ; "1092")]
#[test_case(r#####"tw`border-r-fuchsia-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(245 208 254 / var(--tw-border-opacity))",
})
;"##### ; "1093")]
#[test_case(r#####"tw`border-r-fuchsia-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(240 171 252 / var(--tw-border-opacity))",
})
;"##### ; "1094")]
#[test_case(r#####"tw`border-r-fuchsia-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(232 121 249 / var(--tw-border-opacity))",
})
;"##### ; "1095")]
#[test_case(r#####"tw`border-r-fuchsia-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(217 70 239 / var(--tw-border-opacity))",
})
;"##### ; "1096")]
#[test_case(r#####"tw`border-r-fuchsia-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(192 38 211 / var(--tw-border-opacity))",
})
;"##### ; "1097")]
#[test_case(r#####"tw`border-r-fuchsia-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(162 28 175 / var(--tw-border-opacity))",
})
;"##### ; "1098")]
#[test_case(r#####"tw`border-r-fuchsia-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(134 25 143 / var(--tw-border-opacity))",
})
;"##### ; "1099")]
#[test_case(r#####"tw`border-r-fuchsia-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(112 26 117 / var(--tw-border-opacity))",
})
;"##### ; "1100")]
#[test_case(r#####"tw`border-r-pink-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(253 242 248 / var(--tw-border-opacity))",
})
;"##### ; "1101")]
#[test_case(r#####"tw`border-r-pink-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(252 231 243 / var(--tw-border-opacity))",
})
;"##### ; "1102")]
#[test_case(r#####"tw`border-r-pink-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(251 207 232 / var(--tw-border-opacity))",
})
;"##### ; "1103")]
#[test_case(r#####"tw`border-r-pink-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(249 168 212 / var(--tw-border-opacity))",
})
;"##### ; "1104")]
#[test_case(r#####"tw`border-r-pink-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(244 114 182 / var(--tw-border-opacity))",
})
;"##### ; "1105")]
#[test_case(r#####"tw`border-r-pink-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(236 72 153 / var(--tw-border-opacity))",
})
;"##### ; "1106")]
#[test_case(r#####"tw`border-r-pink-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(219 39 119 / var(--tw-border-opacity))",
})
;"##### ; "1107")]
#[test_case(r#####"tw`border-r-pink-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(190 24 93 / var(--tw-border-opacity))",
})
;"##### ; "1108")]
#[test_case(r#####"tw`border-r-pink-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(157 23 77 / var(--tw-border-opacity))",
})
;"##### ; "1109")]
#[test_case(r#####"tw`border-r-pink-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(131 24 67 / var(--tw-border-opacity))",
})
;"##### ; "1110")]
#[test_case(r#####"tw`border-r-rose-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(255 241 242 / var(--tw-border-opacity))",
})
;"##### ; "1111")]
#[test_case(r#####"tw`border-r-rose-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(255 228 230 / var(--tw-border-opacity))",
})
;"##### ; "1112")]
#[test_case(r#####"tw`border-r-rose-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(254 205 211 / var(--tw-border-opacity))",
})
;"##### ; "1113")]
#[test_case(r#####"tw`border-r-rose-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(253 164 175 / var(--tw-border-opacity))",
})
;"##### ; "1114")]
#[test_case(r#####"tw`border-r-rose-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(251 113 133 / var(--tw-border-opacity))",
})
;"##### ; "1115")]
#[test_case(r#####"tw`border-r-rose-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(244 63 94 / var(--tw-border-opacity))",
})
;"##### ; "1116")]
#[test_case(r#####"tw`border-r-rose-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(225 29 72 / var(--tw-border-opacity))",
})
;"##### ; "1117")]
#[test_case(r#####"tw`border-r-rose-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(190 18 60 / var(--tw-border-opacity))",
})
;"##### ; "1118")]
#[test_case(r#####"tw`border-r-rose-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(159 18 57 / var(--tw-border-opacity))",
})
;"##### ; "1119")]
#[test_case(r#####"tw`border-r-rose-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(136 19 55 / var(--tw-border-opacity))",
})
;"##### ; "1120")]
#[test_case(r#####"tw`border-b-inherit`"#####, r#####"({
  borderBottomColor: "inherit",
})
;"##### ; "1121")]
#[test_case(r#####"tw`border-b-current`"#####, r#####"({
  borderBottomColor: "currentColor",
})
;"##### ; "1122")]
#[test_case(r#####"tw`border-b-transparent`"#####, r#####"({
  borderBottomColor: "transparent",
})
;"##### ; "1123")]
#[test_case(r#####"tw`border-b-black`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(0 0 0 / var(--tw-border-opacity))",
})
;"##### ; "1124")]
#[test_case(r#####"tw`border-b-white`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(255 255 255 / var(--tw-border-opacity))",
})
;"##### ; "1125")]
#[test_case(r#####"tw`border-b-slate-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(248 250 252 / var(--tw-border-opacity))",
})
;"##### ; "1126")]
#[test_case(r#####"tw`border-b-slate-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(241 245 249 / var(--tw-border-opacity))",
})
;"##### ; "1127")]
#[test_case(r#####"tw`border-b-slate-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(226 232 240 / var(--tw-border-opacity))",
})
;"##### ; "1128")]
#[test_case(r#####"tw`border-b-slate-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(203 213 225 / var(--tw-border-opacity))",
})
;"##### ; "1129")]
#[test_case(r#####"tw`border-b-slate-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(148 163 184 / var(--tw-border-opacity))",
})
;"##### ; "1130")]
#[test_case(r#####"tw`border-b-slate-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(100 116 139 / var(--tw-border-opacity))",
})
;"##### ; "1131")]
#[test_case(r#####"tw`border-b-slate-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(71 85 105 / var(--tw-border-opacity))",
})
;"##### ; "1132")]
#[test_case(r#####"tw`border-b-slate-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(51 65 85 / var(--tw-border-opacity))",
})
;"##### ; "1133")]
#[test_case(r#####"tw`border-b-slate-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(30 41 59 / var(--tw-border-opacity))",
})
;"##### ; "1134")]
#[test_case(r#####"tw`border-b-slate-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(15 23 42 / var(--tw-border-opacity))",
})
;"##### ; "1135")]
#[test_case(r#####"tw`border-b-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "1136")]
#[test_case(r#####"tw`border-b-gray-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(243 244 246 / var(--tw-border-opacity))",
})
;"##### ; "1137")]
#[test_case(r#####"tw`border-b-gray-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(229 231 235 / var(--tw-border-opacity))",
})
;"##### ; "1138")]
#[test_case(r#####"tw`border-b-gray-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(209 213 219 / var(--tw-border-opacity))",
})
;"##### ; "1139")]
#[test_case(r#####"tw`border-b-gray-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(156 163 175 / var(--tw-border-opacity))",
})
;"##### ; "1140")]
#[test_case(r#####"tw`border-b-gray-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(107 114 128 / var(--tw-border-opacity))",
})
;"##### ; "1141")]
#[test_case(r#####"tw`border-b-gray-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(75 85 99 / var(--tw-border-opacity))",
})
;"##### ; "1142")]
#[test_case(r#####"tw`border-b-gray-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(55 65 81 / var(--tw-border-opacity))",
})
;"##### ; "1143")]
#[test_case(r#####"tw`border-b-gray-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(31 41 55 / var(--tw-border-opacity))",
})
;"##### ; "1144")]
#[test_case(r#####"tw`border-b-gray-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(17 24 39 / var(--tw-border-opacity))",
})
;"##### ; "1145")]
#[test_case(r#####"tw`border-b-zinc-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "1146")]
#[test_case(r#####"tw`border-b-zinc-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(244 244 245 / var(--tw-border-opacity))",
})
;"##### ; "1147")]
#[test_case(r#####"tw`border-b-zinc-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(228 228 231 / var(--tw-border-opacity))",
})
;"##### ; "1148")]
#[test_case(r#####"tw`border-b-zinc-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(212 212 216 / var(--tw-border-opacity))",
})
;"##### ; "1149")]
#[test_case(r#####"tw`border-b-zinc-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(161 161 170 / var(--tw-border-opacity))",
})
;"##### ; "1150")]
#[test_case(r#####"tw`border-b-zinc-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(113 113 122 / var(--tw-border-opacity))",
})
;"##### ; "1151")]
#[test_case(r#####"tw`border-b-zinc-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(82 82 91 / var(--tw-border-opacity))",
})
;"##### ; "1152")]
#[test_case(r#####"tw`border-b-zinc-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(63 63 70 / var(--tw-border-opacity))",
})
;"##### ; "1153")]
#[test_case(r#####"tw`border-b-zinc-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(39 39 42 / var(--tw-border-opacity))",
})
;"##### ; "1154")]
#[test_case(r#####"tw`border-b-zinc-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(24 24 27 / var(--tw-border-opacity))",
})
;"##### ; "1155")]
#[test_case(r#####"tw`border-b-neutral-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "1156")]
#[test_case(r#####"tw`border-b-neutral-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(245 245 245 / var(--tw-border-opacity))",
})
;"##### ; "1157")]
#[test_case(r#####"tw`border-b-neutral-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(229 229 229 / var(--tw-border-opacity))",
})
;"##### ; "1158")]
#[test_case(r#####"tw`border-b-neutral-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(212 212 212 / var(--tw-border-opacity))",
})
;"##### ; "1159")]
#[test_case(r#####"tw`border-b-neutral-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(163 163 163 / var(--tw-border-opacity))",
})
;"##### ; "1160")]
#[test_case(r#####"tw`border-b-neutral-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(115 115 115 / var(--tw-border-opacity))",
})
;"##### ; "1161")]
#[test_case(r#####"tw`border-b-neutral-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(82 82 82 / var(--tw-border-opacity))",
})
;"##### ; "1162")]
#[test_case(r#####"tw`border-b-neutral-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(64 64 64 / var(--tw-border-opacity))",
})
;"##### ; "1163")]
#[test_case(r#####"tw`border-b-neutral-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(38 38 38 / var(--tw-border-opacity))",
})
;"##### ; "1164")]
#[test_case(r#####"tw`border-b-neutral-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(23 23 23 / var(--tw-border-opacity))",
})
;"##### ; "1165")]
#[test_case(r#####"tw`border-b-stone-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(250 250 249 / var(--tw-border-opacity))",
})
;"##### ; "1166")]
#[test_case(r#####"tw`border-b-stone-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(245 245 244 / var(--tw-border-opacity))",
})
;"##### ; "1167")]
#[test_case(r#####"tw`border-b-stone-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(231 229 228 / var(--tw-border-opacity))",
})
;"##### ; "1168")]
#[test_case(r#####"tw`border-b-stone-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(214 211 209 / var(--tw-border-opacity))",
})
;"##### ; "1169")]
#[test_case(r#####"tw`border-b-stone-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(168 162 158 / var(--tw-border-opacity))",
})
;"##### ; "1170")]
#[test_case(r#####"tw`border-b-stone-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(120 113 108 / var(--tw-border-opacity))",
})
;"##### ; "1171")]
#[test_case(r#####"tw`border-b-stone-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(87 83 78 / var(--tw-border-opacity))",
})
;"##### ; "1172")]
#[test_case(r#####"tw`border-b-stone-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(68 64 60 / var(--tw-border-opacity))",
})
;"##### ; "1173")]
#[test_case(r#####"tw`border-b-stone-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(41 37 36 / var(--tw-border-opacity))",
})
;"##### ; "1174")]
#[test_case(r#####"tw`border-b-stone-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(28 25 23 / var(--tw-border-opacity))",
})
;"##### ; "1175")]
#[test_case(r#####"tw`border-b-red-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(254 242 242 / var(--tw-border-opacity))",
})
;"##### ; "1176")]
#[test_case(r#####"tw`border-b-red-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(254 226 226 / var(--tw-border-opacity))",
})
;"##### ; "1177")]
#[test_case(r#####"tw`border-b-red-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(254 202 202 / var(--tw-border-opacity))",
})
;"##### ; "1178")]
#[test_case(r#####"tw`border-b-red-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(252 165 165 / var(--tw-border-opacity))",
})
;"##### ; "1179")]
#[test_case(r#####"tw`border-b-red-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(248 113 113 / var(--tw-border-opacity))",
})
;"##### ; "1180")]
#[test_case(r#####"tw`border-b-red-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(239 68 68 / var(--tw-border-opacity))",
})
;"##### ; "1181")]
#[test_case(r#####"tw`border-b-red-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(220 38 38 / var(--tw-border-opacity))",
})
;"##### ; "1182")]
#[test_case(r#####"tw`border-b-red-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(185 28 28 / var(--tw-border-opacity))",
})
;"##### ; "1183")]
#[test_case(r#####"tw`border-b-red-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(153 27 27 / var(--tw-border-opacity))",
})
;"##### ; "1184")]
#[test_case(r#####"tw`border-b-red-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(127 29 29 / var(--tw-border-opacity))",
})
;"##### ; "1185")]
#[test_case(r#####"tw`border-b-orange-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(255 247 237 / var(--tw-border-opacity))",
})
;"##### ; "1186")]
#[test_case(r#####"tw`border-b-orange-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(255 237 213 / var(--tw-border-opacity))",
})
;"##### ; "1187")]
#[test_case(r#####"tw`border-b-orange-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(254 215 170 / var(--tw-border-opacity))",
})
;"##### ; "1188")]
#[test_case(r#####"tw`border-b-orange-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(253 186 116 / var(--tw-border-opacity))",
})
;"##### ; "1189")]
#[test_case(r#####"tw`border-b-orange-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(251 146 60 / var(--tw-border-opacity))",
})
;"##### ; "1190")]
#[test_case(r#####"tw`border-b-orange-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(249 115 22 / var(--tw-border-opacity))",
})
;"##### ; "1191")]
#[test_case(r#####"tw`border-b-orange-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(234 88 12 / var(--tw-border-opacity))",
})
;"##### ; "1192")]
#[test_case(r#####"tw`border-b-orange-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(194 65 12 / var(--tw-border-opacity))",
})
;"##### ; "1193")]
#[test_case(r#####"tw`border-b-orange-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(154 52 18 / var(--tw-border-opacity))",
})
;"##### ; "1194")]
#[test_case(r#####"tw`border-b-orange-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(124 45 18 / var(--tw-border-opacity))",
})
;"##### ; "1195")]
#[test_case(r#####"tw`border-b-amber-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(255 251 235 / var(--tw-border-opacity))",
})
;"##### ; "1196")]
#[test_case(r#####"tw`border-b-amber-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(254 243 199 / var(--tw-border-opacity))",
})
;"##### ; "1197")]
#[test_case(r#####"tw`border-b-amber-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(253 230 138 / var(--tw-border-opacity))",
})
;"##### ; "1198")]
#[test_case(r#####"tw`border-b-amber-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(252 211 77 / var(--tw-border-opacity))",
})
;"##### ; "1199")]
#[test_case(r#####"tw`border-b-amber-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(251 191 36 / var(--tw-border-opacity))",
})
;"##### ; "1200")]
#[test_case(r#####"tw`border-b-amber-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(245 158 11 / var(--tw-border-opacity))",
})
;"##### ; "1201")]
#[test_case(r#####"tw`border-b-amber-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(217 119 6 / var(--tw-border-opacity))",
})
;"##### ; "1202")]
#[test_case(r#####"tw`border-b-amber-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(180 83 9 / var(--tw-border-opacity))",
})
;"##### ; "1203")]
#[test_case(r#####"tw`border-b-amber-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(146 64 14 / var(--tw-border-opacity))",
})
;"##### ; "1204")]
#[test_case(r#####"tw`border-b-amber-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(120 53 15 / var(--tw-border-opacity))",
})
;"##### ; "1205")]
#[test_case(r#####"tw`border-b-yellow-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(254 252 232 / var(--tw-border-opacity))",
})
;"##### ; "1206")]
#[test_case(r#####"tw`border-b-yellow-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(254 249 195 / var(--tw-border-opacity))",
})
;"##### ; "1207")]
#[test_case(r#####"tw`border-b-yellow-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(254 240 138 / var(--tw-border-opacity))",
})
;"##### ; "1208")]
#[test_case(r#####"tw`border-b-yellow-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(253 224 71 / var(--tw-border-opacity))",
})
;"##### ; "1209")]
#[test_case(r#####"tw`border-b-yellow-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(250 204 21 / var(--tw-border-opacity))",
})
;"##### ; "1210")]
#[test_case(r#####"tw`border-b-yellow-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(234 179 8 / var(--tw-border-opacity))",
})
;"##### ; "1211")]
#[test_case(r#####"tw`border-b-yellow-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(202 138 4 / var(--tw-border-opacity))",
})
;"##### ; "1212")]
#[test_case(r#####"tw`border-b-yellow-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(161 98 7 / var(--tw-border-opacity))",
})
;"##### ; "1213")]
#[test_case(r#####"tw`border-b-yellow-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(133 77 14 / var(--tw-border-opacity))",
})
;"##### ; "1214")]
#[test_case(r#####"tw`border-b-yellow-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(113 63 18 / var(--tw-border-opacity))",
})
;"##### ; "1215")]
#[test_case(r#####"tw`border-b-lime-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(247 254 231 / var(--tw-border-opacity))",
})
;"##### ; "1216")]
#[test_case(r#####"tw`border-b-lime-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(236 252 203 / var(--tw-border-opacity))",
})
;"##### ; "1217")]
#[test_case(r#####"tw`border-b-lime-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(217 249 157 / var(--tw-border-opacity))",
})
;"##### ; "1218")]
#[test_case(r#####"tw`border-b-lime-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(190 242 100 / var(--tw-border-opacity))",
})
;"##### ; "1219")]
#[test_case(r#####"tw`border-b-lime-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(163 230 53 / var(--tw-border-opacity))",
})
;"##### ; "1220")]
#[test_case(r#####"tw`border-b-lime-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(132 204 22 / var(--tw-border-opacity))",
})
;"##### ; "1221")]
#[test_case(r#####"tw`border-b-lime-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(101 163 13 / var(--tw-border-opacity))",
})
;"##### ; "1222")]
#[test_case(r#####"tw`border-b-lime-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(77 124 15 / var(--tw-border-opacity))",
})
;"##### ; "1223")]
#[test_case(r#####"tw`border-b-lime-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(63 98 18 / var(--tw-border-opacity))",
})
;"##### ; "1224")]
#[test_case(r#####"tw`border-b-lime-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(54 83 20 / var(--tw-border-opacity))",
})
;"##### ; "1225")]
#[test_case(r#####"tw`border-b-green-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(240 253 244 / var(--tw-border-opacity))",
})
;"##### ; "1226")]
#[test_case(r#####"tw`border-b-green-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(220 252 231 / var(--tw-border-opacity))",
})
;"##### ; "1227")]
#[test_case(r#####"tw`border-b-green-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(187 247 208 / var(--tw-border-opacity))",
})
;"##### ; "1228")]
#[test_case(r#####"tw`border-b-green-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(134 239 172 / var(--tw-border-opacity))",
})
;"##### ; "1229")]
#[test_case(r#####"tw`border-b-green-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(74 222 128 / var(--tw-border-opacity))",
})
;"##### ; "1230")]
#[test_case(r#####"tw`border-b-green-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(34 197 94 / var(--tw-border-opacity))",
})
;"##### ; "1231")]
#[test_case(r#####"tw`border-b-green-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(22 163 74 / var(--tw-border-opacity))",
})
;"##### ; "1232")]
#[test_case(r#####"tw`border-b-green-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(21 128 61 / var(--tw-border-opacity))",
})
;"##### ; "1233")]
#[test_case(r#####"tw`border-b-green-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(22 101 52 / var(--tw-border-opacity))",
})
;"##### ; "1234")]
#[test_case(r#####"tw`border-b-green-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(20 83 45 / var(--tw-border-opacity))",
})
;"##### ; "1235")]
#[test_case(r#####"tw`border-b-emerald-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(236 253 245 / var(--tw-border-opacity))",
})
;"##### ; "1236")]
#[test_case(r#####"tw`border-b-emerald-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(209 250 229 / var(--tw-border-opacity))",
})
;"##### ; "1237")]
#[test_case(r#####"tw`border-b-emerald-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(167 243 208 / var(--tw-border-opacity))",
})
;"##### ; "1238")]
#[test_case(r#####"tw`border-b-emerald-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(110 231 183 / var(--tw-border-opacity))",
})
;"##### ; "1239")]
#[test_case(r#####"tw`border-b-emerald-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(52 211 153 / var(--tw-border-opacity))",
})
;"##### ; "1240")]
#[test_case(r#####"tw`border-b-emerald-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(16 185 129 / var(--tw-border-opacity))",
})
;"##### ; "1241")]
#[test_case(r#####"tw`border-b-emerald-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(5 150 105 / var(--tw-border-opacity))",
})
;"##### ; "1242")]
#[test_case(r#####"tw`border-b-emerald-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(4 120 87 / var(--tw-border-opacity))",
})
;"##### ; "1243")]
#[test_case(r#####"tw`border-b-emerald-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(6 95 70 / var(--tw-border-opacity))",
})
;"##### ; "1244")]
#[test_case(r#####"tw`border-b-emerald-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(6 78 59 / var(--tw-border-opacity))",
})
;"##### ; "1245")]
#[test_case(r#####"tw`border-b-teal-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(240 253 250 / var(--tw-border-opacity))",
})
;"##### ; "1246")]
#[test_case(r#####"tw`border-b-teal-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(204 251 241 / var(--tw-border-opacity))",
})
;"##### ; "1247")]
#[test_case(r#####"tw`border-b-teal-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(153 246 228 / var(--tw-border-opacity))",
})
;"##### ; "1248")]
#[test_case(r#####"tw`border-b-teal-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(94 234 212 / var(--tw-border-opacity))",
})
;"##### ; "1249")]
#[test_case(r#####"tw`border-b-teal-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(45 212 191 / var(--tw-border-opacity))",
})
;"##### ; "1250")]
#[test_case(r#####"tw`border-b-teal-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(20 184 166 / var(--tw-border-opacity))",
})
;"##### ; "1251")]
#[test_case(r#####"tw`border-b-teal-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(13 148 136 / var(--tw-border-opacity))",
})
;"##### ; "1252")]
#[test_case(r#####"tw`border-b-teal-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(15 118 110 / var(--tw-border-opacity))",
})
;"##### ; "1253")]
#[test_case(r#####"tw`border-b-teal-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(17 94 89 / var(--tw-border-opacity))",
})
;"##### ; "1254")]
#[test_case(r#####"tw`border-b-teal-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(19 78 74 / var(--tw-border-opacity))",
})
;"##### ; "1255")]
#[test_case(r#####"tw`border-b-cyan-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(236 254 255 / var(--tw-border-opacity))",
})
;"##### ; "1256")]
#[test_case(r#####"tw`border-b-cyan-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(207 250 254 / var(--tw-border-opacity))",
})
;"##### ; "1257")]
#[test_case(r#####"tw`border-b-cyan-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(165 243 252 / var(--tw-border-opacity))",
})
;"##### ; "1258")]
#[test_case(r#####"tw`border-b-cyan-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(103 232 249 / var(--tw-border-opacity))",
})
;"##### ; "1259")]
#[test_case(r#####"tw`border-b-cyan-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(34 211 238 / var(--tw-border-opacity))",
})
;"##### ; "1260")]
#[test_case(r#####"tw`border-b-cyan-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(6 182 212 / var(--tw-border-opacity))",
})
;"##### ; "1261")]
#[test_case(r#####"tw`border-b-cyan-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(8 145 178 / var(--tw-border-opacity))",
})
;"##### ; "1262")]
#[test_case(r#####"tw`border-b-cyan-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(14 116 144 / var(--tw-border-opacity))",
})
;"##### ; "1263")]
#[test_case(r#####"tw`border-b-cyan-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(21 94 117 / var(--tw-border-opacity))",
})
;"##### ; "1264")]
#[test_case(r#####"tw`border-b-cyan-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(22 78 99 / var(--tw-border-opacity))",
})
;"##### ; "1265")]
#[test_case(r#####"tw`border-b-sky-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(240 249 255 / var(--tw-border-opacity))",
})
;"##### ; "1266")]
#[test_case(r#####"tw`border-b-sky-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(224 242 254 / var(--tw-border-opacity))",
})
;"##### ; "1267")]
#[test_case(r#####"tw`border-b-sky-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(186 230 253 / var(--tw-border-opacity))",
})
;"##### ; "1268")]
#[test_case(r#####"tw`border-b-sky-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(125 211 252 / var(--tw-border-opacity))",
})
;"##### ; "1269")]
#[test_case(r#####"tw`border-b-sky-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(56 189 248 / var(--tw-border-opacity))",
})
;"##### ; "1270")]
#[test_case(r#####"tw`border-b-sky-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(14 165 233 / var(--tw-border-opacity))",
})
;"##### ; "1271")]
#[test_case(r#####"tw`border-b-sky-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(2 132 199 / var(--tw-border-opacity))",
})
;"##### ; "1272")]
#[test_case(r#####"tw`border-b-sky-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(3 105 161 / var(--tw-border-opacity))",
})
;"##### ; "1273")]
#[test_case(r#####"tw`border-b-sky-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(7 89 133 / var(--tw-border-opacity))",
})
;"##### ; "1274")]
#[test_case(r#####"tw`border-b-sky-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(12 74 110 / var(--tw-border-opacity))",
})
;"##### ; "1275")]
#[test_case(r#####"tw`border-b-blue-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(239 246 255 / var(--tw-border-opacity))",
})
;"##### ; "1276")]
#[test_case(r#####"tw`border-b-blue-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(219 234 254 / var(--tw-border-opacity))",
})
;"##### ; "1277")]
#[test_case(r#####"tw`border-b-blue-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(191 219 254 / var(--tw-border-opacity))",
})
;"##### ; "1278")]
#[test_case(r#####"tw`border-b-blue-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(147 197 253 / var(--tw-border-opacity))",
})
;"##### ; "1279")]
#[test_case(r#####"tw`border-b-blue-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(96 165 250 / var(--tw-border-opacity))",
})
;"##### ; "1280")]
#[test_case(r#####"tw`border-b-blue-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(59 130 246 / var(--tw-border-opacity))",
})
;"##### ; "1281")]
#[test_case(r#####"tw`border-b-blue-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(37 99 235 / var(--tw-border-opacity))",
})
;"##### ; "1282")]
#[test_case(r#####"tw`border-b-blue-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(29 78 216 / var(--tw-border-opacity))",
})
;"##### ; "1283")]
#[test_case(r#####"tw`border-b-blue-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(30 64 175 / var(--tw-border-opacity))",
})
;"##### ; "1284")]
#[test_case(r#####"tw`border-b-blue-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(30 58 138 / var(--tw-border-opacity))",
})
;"##### ; "1285")]
#[test_case(r#####"tw`border-b-indigo-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(238 242 255 / var(--tw-border-opacity))",
})
;"##### ; "1286")]
#[test_case(r#####"tw`border-b-indigo-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(224 231 255 / var(--tw-border-opacity))",
})
;"##### ; "1287")]
#[test_case(r#####"tw`border-b-indigo-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(199 210 254 / var(--tw-border-opacity))",
})
;"##### ; "1288")]
#[test_case(r#####"tw`border-b-indigo-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(165 180 252 / var(--tw-border-opacity))",
})
;"##### ; "1289")]
#[test_case(r#####"tw`border-b-indigo-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(129 140 248 / var(--tw-border-opacity))",
})
;"##### ; "1290")]
#[test_case(r#####"tw`border-b-indigo-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(99 102 241 / var(--tw-border-opacity))",
})
;"##### ; "1291")]
#[test_case(r#####"tw`border-b-indigo-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(79 70 229 / var(--tw-border-opacity))",
})
;"##### ; "1292")]
#[test_case(r#####"tw`border-b-indigo-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(67 56 202 / var(--tw-border-opacity))",
})
;"##### ; "1293")]
#[test_case(r#####"tw`border-b-indigo-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(55 48 163 / var(--tw-border-opacity))",
})
;"##### ; "1294")]
#[test_case(r#####"tw`border-b-indigo-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(49 46 129 / var(--tw-border-opacity))",
})
;"##### ; "1295")]
#[test_case(r#####"tw`border-b-violet-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(245 243 255 / var(--tw-border-opacity))",
})
;"##### ; "1296")]
#[test_case(r#####"tw`border-b-violet-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(237 233 254 / var(--tw-border-opacity))",
})
;"##### ; "1297")]
#[test_case(r#####"tw`border-b-violet-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(221 214 254 / var(--tw-border-opacity))",
})
;"##### ; "1298")]
#[test_case(r#####"tw`border-b-violet-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(196 181 253 / var(--tw-border-opacity))",
})
;"##### ; "1299")]
#[test_case(r#####"tw`border-b-violet-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(167 139 250 / var(--tw-border-opacity))",
})
;"##### ; "1300")]
#[test_case(r#####"tw`border-b-violet-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(139 92 246 / var(--tw-border-opacity))",
})
;"##### ; "1301")]
#[test_case(r#####"tw`border-b-violet-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(124 58 237 / var(--tw-border-opacity))",
})
;"##### ; "1302")]
#[test_case(r#####"tw`border-b-violet-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(109 40 217 / var(--tw-border-opacity))",
})
;"##### ; "1303")]
#[test_case(r#####"tw`border-b-violet-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(91 33 182 / var(--tw-border-opacity))",
})
;"##### ; "1304")]
#[test_case(r#####"tw`border-b-violet-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(76 29 149 / var(--tw-border-opacity))",
})
;"##### ; "1305")]
#[test_case(r#####"tw`border-b-purple-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(250 245 255 / var(--tw-border-opacity))",
})
;"##### ; "1306")]
#[test_case(r#####"tw`border-b-purple-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(243 232 255 / var(--tw-border-opacity))",
})
;"##### ; "1307")]
#[test_case(r#####"tw`border-b-purple-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(233 213 255 / var(--tw-border-opacity))",
})
;"##### ; "1308")]
#[test_case(r#####"tw`border-b-purple-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(216 180 254 / var(--tw-border-opacity))",
})
;"##### ; "1309")]
#[test_case(r#####"tw`border-b-purple-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(192 132 252 / var(--tw-border-opacity))",
})
;"##### ; "1310")]
#[test_case(r#####"tw`border-b-purple-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(168 85 247 / var(--tw-border-opacity))",
})
;"##### ; "1311")]
#[test_case(r#####"tw`border-b-purple-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(147 51 234 / var(--tw-border-opacity))",
})
;"##### ; "1312")]
#[test_case(r#####"tw`border-b-purple-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(126 34 206 / var(--tw-border-opacity))",
})
;"##### ; "1313")]
#[test_case(r#####"tw`border-b-purple-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(107 33 168 / var(--tw-border-opacity))",
})
;"##### ; "1314")]
#[test_case(r#####"tw`border-b-purple-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(88 28 135 / var(--tw-border-opacity))",
})
;"##### ; "1315")]
#[test_case(r#####"tw`border-b-fuchsia-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(253 244 255 / var(--tw-border-opacity))",
})
;"##### ; "1316")]
#[test_case(r#####"tw`border-b-fuchsia-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(250 232 255 / var(--tw-border-opacity))",
})
;"##### ; "1317")]
#[test_case(r#####"tw`border-b-fuchsia-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(245 208 254 / var(--tw-border-opacity))",
})
;"##### ; "1318")]
#[test_case(r#####"tw`border-b-fuchsia-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(240 171 252 / var(--tw-border-opacity))",
})
;"##### ; "1319")]
#[test_case(r#####"tw`border-b-fuchsia-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(232 121 249 / var(--tw-border-opacity))",
})
;"##### ; "1320")]
#[test_case(r#####"tw`border-b-fuchsia-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(217 70 239 / var(--tw-border-opacity))",
})
;"##### ; "1321")]
#[test_case(r#####"tw`border-b-fuchsia-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(192 38 211 / var(--tw-border-opacity))",
})
;"##### ; "1322")]
#[test_case(r#####"tw`border-b-fuchsia-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(162 28 175 / var(--tw-border-opacity))",
})
;"##### ; "1323")]
#[test_case(r#####"tw`border-b-fuchsia-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(134 25 143 / var(--tw-border-opacity))",
})
;"##### ; "1324")]
#[test_case(r#####"tw`border-b-fuchsia-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(112 26 117 / var(--tw-border-opacity))",
})
;"##### ; "1325")]
#[test_case(r#####"tw`border-b-pink-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(253 242 248 / var(--tw-border-opacity))",
})
;"##### ; "1326")]
#[test_case(r#####"tw`border-b-pink-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(252 231 243 / var(--tw-border-opacity))",
})
;"##### ; "1327")]
#[test_case(r#####"tw`border-b-pink-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(251 207 232 / var(--tw-border-opacity))",
})
;"##### ; "1328")]
#[test_case(r#####"tw`border-b-pink-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(249 168 212 / var(--tw-border-opacity))",
})
;"##### ; "1329")]
#[test_case(r#####"tw`border-b-pink-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(244 114 182 / var(--tw-border-opacity))",
})
;"##### ; "1330")]
#[test_case(r#####"tw`border-b-pink-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(236 72 153 / var(--tw-border-opacity))",
})
;"##### ; "1331")]
#[test_case(r#####"tw`border-b-pink-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(219 39 119 / var(--tw-border-opacity))",
})
;"##### ; "1332")]
#[test_case(r#####"tw`border-b-pink-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(190 24 93 / var(--tw-border-opacity))",
})
;"##### ; "1333")]
#[test_case(r#####"tw`border-b-pink-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(157 23 77 / var(--tw-border-opacity))",
})
;"##### ; "1334")]
#[test_case(r#####"tw`border-b-pink-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(131 24 67 / var(--tw-border-opacity))",
})
;"##### ; "1335")]
#[test_case(r#####"tw`border-b-rose-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(255 241 242 / var(--tw-border-opacity))",
})
;"##### ; "1336")]
#[test_case(r#####"tw`border-b-rose-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(255 228 230 / var(--tw-border-opacity))",
})
;"##### ; "1337")]
#[test_case(r#####"tw`border-b-rose-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(254 205 211 / var(--tw-border-opacity))",
})
;"##### ; "1338")]
#[test_case(r#####"tw`border-b-rose-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(253 164 175 / var(--tw-border-opacity))",
})
;"##### ; "1339")]
#[test_case(r#####"tw`border-b-rose-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(251 113 133 / var(--tw-border-opacity))",
})
;"##### ; "1340")]
#[test_case(r#####"tw`border-b-rose-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(244 63 94 / var(--tw-border-opacity))",
})
;"##### ; "1341")]
#[test_case(r#####"tw`border-b-rose-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(225 29 72 / var(--tw-border-opacity))",
})
;"##### ; "1342")]
#[test_case(r#####"tw`border-b-rose-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(190 18 60 / var(--tw-border-opacity))",
})
;"##### ; "1343")]
#[test_case(r#####"tw`border-b-rose-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(159 18 57 / var(--tw-border-opacity))",
})
;"##### ; "1344")]
#[test_case(r#####"tw`border-b-rose-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(136 19 55 / var(--tw-border-opacity))",
})
;"##### ; "1345")]
#[test_case(r#####"tw`border-l-inherit`"#####, r#####"({
  borderLeftColor: "inherit",
})
;"##### ; "1346")]
#[test_case(r#####"tw`border-l-current`"#####, r#####"({
  borderLeftColor: "currentColor",
})
;"##### ; "1347")]
#[test_case(r#####"tw`border-l-transparent`"#####, r#####"({
  borderLeftColor: "transparent",
})
;"##### ; "1348")]
#[test_case(r#####"tw`border-l-black`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(0 0 0 / var(--tw-border-opacity))",
})
;"##### ; "1349")]
#[test_case(r#####"tw`border-l-white`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(255 255 255 / var(--tw-border-opacity))",
})
;"##### ; "1350")]
#[test_case(r#####"tw`border-l-slate-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(248 250 252 / var(--tw-border-opacity))",
})
;"##### ; "1351")]
#[test_case(r#####"tw`border-l-slate-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(241 245 249 / var(--tw-border-opacity))",
})
;"##### ; "1352")]
#[test_case(r#####"tw`border-l-slate-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(226 232 240 / var(--tw-border-opacity))",
})
;"##### ; "1353")]
#[test_case(r#####"tw`border-l-slate-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(203 213 225 / var(--tw-border-opacity))",
})
;"##### ; "1354")]
#[test_case(r#####"tw`border-l-slate-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(148 163 184 / var(--tw-border-opacity))",
})
;"##### ; "1355")]
#[test_case(r#####"tw`border-l-slate-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(100 116 139 / var(--tw-border-opacity))",
})
;"##### ; "1356")]
#[test_case(r#####"tw`border-l-slate-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(71 85 105 / var(--tw-border-opacity))",
})
;"##### ; "1357")]
#[test_case(r#####"tw`border-l-slate-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(51 65 85 / var(--tw-border-opacity))",
})
;"##### ; "1358")]
#[test_case(r#####"tw`border-l-slate-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(30 41 59 / var(--tw-border-opacity))",
})
;"##### ; "1359")]
#[test_case(r#####"tw`border-l-slate-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(15 23 42 / var(--tw-border-opacity))",
})
;"##### ; "1360")]
#[test_case(r#####"tw`border-l-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "1361")]
#[test_case(r#####"tw`border-l-gray-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(243 244 246 / var(--tw-border-opacity))",
})
;"##### ; "1362")]
#[test_case(r#####"tw`border-l-gray-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(229 231 235 / var(--tw-border-opacity))",
})
;"##### ; "1363")]
#[test_case(r#####"tw`border-l-gray-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(209 213 219 / var(--tw-border-opacity))",
})
;"##### ; "1364")]
#[test_case(r#####"tw`border-l-gray-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(156 163 175 / var(--tw-border-opacity))",
})
;"##### ; "1365")]
#[test_case(r#####"tw`border-l-gray-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(107 114 128 / var(--tw-border-opacity))",
})
;"##### ; "1366")]
#[test_case(r#####"tw`border-l-gray-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(75 85 99 / var(--tw-border-opacity))",
})
;"##### ; "1367")]
#[test_case(r#####"tw`border-l-gray-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(55 65 81 / var(--tw-border-opacity))",
})
;"##### ; "1368")]
#[test_case(r#####"tw`border-l-gray-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(31 41 55 / var(--tw-border-opacity))",
})
;"##### ; "1369")]
#[test_case(r#####"tw`border-l-gray-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(17 24 39 / var(--tw-border-opacity))",
})
;"##### ; "1370")]
#[test_case(r#####"tw`border-l-zinc-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "1371")]
#[test_case(r#####"tw`border-l-zinc-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(244 244 245 / var(--tw-border-opacity))",
})
;"##### ; "1372")]
#[test_case(r#####"tw`border-l-zinc-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(228 228 231 / var(--tw-border-opacity))",
})
;"##### ; "1373")]
#[test_case(r#####"tw`border-l-zinc-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(212 212 216 / var(--tw-border-opacity))",
})
;"##### ; "1374")]
#[test_case(r#####"tw`border-l-zinc-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(161 161 170 / var(--tw-border-opacity))",
})
;"##### ; "1375")]
#[test_case(r#####"tw`border-l-zinc-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(113 113 122 / var(--tw-border-opacity))",
})
;"##### ; "1376")]
#[test_case(r#####"tw`border-l-zinc-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(82 82 91 / var(--tw-border-opacity))",
})
;"##### ; "1377")]
#[test_case(r#####"tw`border-l-zinc-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(63 63 70 / var(--tw-border-opacity))",
})
;"##### ; "1378")]
#[test_case(r#####"tw`border-l-zinc-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(39 39 42 / var(--tw-border-opacity))",
})
;"##### ; "1379")]
#[test_case(r#####"tw`border-l-zinc-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(24 24 27 / var(--tw-border-opacity))",
})
;"##### ; "1380")]
#[test_case(r#####"tw`border-l-neutral-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(250 250 250 / var(--tw-border-opacity))",
})
;"##### ; "1381")]
#[test_case(r#####"tw`border-l-neutral-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(245 245 245 / var(--tw-border-opacity))",
})
;"##### ; "1382")]
#[test_case(r#####"tw`border-l-neutral-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(229 229 229 / var(--tw-border-opacity))",
})
;"##### ; "1383")]
#[test_case(r#####"tw`border-l-neutral-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(212 212 212 / var(--tw-border-opacity))",
})
;"##### ; "1384")]
#[test_case(r#####"tw`border-l-neutral-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(163 163 163 / var(--tw-border-opacity))",
})
;"##### ; "1385")]
#[test_case(r#####"tw`border-l-neutral-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(115 115 115 / var(--tw-border-opacity))",
})
;"##### ; "1386")]
#[test_case(r#####"tw`border-l-neutral-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(82 82 82 / var(--tw-border-opacity))",
})
;"##### ; "1387")]
#[test_case(r#####"tw`border-l-neutral-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(64 64 64 / var(--tw-border-opacity))",
})
;"##### ; "1388")]
#[test_case(r#####"tw`border-l-neutral-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(38 38 38 / var(--tw-border-opacity))",
})
;"##### ; "1389")]
#[test_case(r#####"tw`border-l-neutral-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(23 23 23 / var(--tw-border-opacity))",
})
;"##### ; "1390")]
#[test_case(r#####"tw`border-l-stone-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(250 250 249 / var(--tw-border-opacity))",
})
;"##### ; "1391")]
#[test_case(r#####"tw`border-l-stone-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(245 245 244 / var(--tw-border-opacity))",
})
;"##### ; "1392")]
#[test_case(r#####"tw`border-l-stone-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(231 229 228 / var(--tw-border-opacity))",
})
;"##### ; "1393")]
#[test_case(r#####"tw`border-l-stone-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(214 211 209 / var(--tw-border-opacity))",
})
;"##### ; "1394")]
#[test_case(r#####"tw`border-l-stone-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(168 162 158 / var(--tw-border-opacity))",
})
;"##### ; "1395")]
#[test_case(r#####"tw`border-l-stone-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(120 113 108 / var(--tw-border-opacity))",
})
;"##### ; "1396")]
#[test_case(r#####"tw`border-l-stone-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(87 83 78 / var(--tw-border-opacity))",
})
;"##### ; "1397")]
#[test_case(r#####"tw`border-l-stone-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(68 64 60 / var(--tw-border-opacity))",
})
;"##### ; "1398")]
#[test_case(r#####"tw`border-l-stone-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(41 37 36 / var(--tw-border-opacity))",
})
;"##### ; "1399")]
#[test_case(r#####"tw`border-l-stone-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(28 25 23 / var(--tw-border-opacity))",
})
;"##### ; "1400")]
#[test_case(r#####"tw`border-l-red-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 242 242 / var(--tw-border-opacity))",
})
;"##### ; "1401")]
#[test_case(r#####"tw`border-l-red-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 226 226 / var(--tw-border-opacity))",
})
;"##### ; "1402")]
#[test_case(r#####"tw`border-l-red-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 202 202 / var(--tw-border-opacity))",
})
;"##### ; "1403")]
#[test_case(r#####"tw`border-l-red-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(252 165 165 / var(--tw-border-opacity))",
})
;"##### ; "1404")]
#[test_case(r#####"tw`border-l-red-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(248 113 113 / var(--tw-border-opacity))",
})
;"##### ; "1405")]
#[test_case(r#####"tw`border-l-red-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(239 68 68 / var(--tw-border-opacity))",
})
;"##### ; "1406")]
#[test_case(r#####"tw`border-l-red-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(220 38 38 / var(--tw-border-opacity))",
})
;"##### ; "1407")]
#[test_case(r#####"tw`border-l-red-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(185 28 28 / var(--tw-border-opacity))",
})
;"##### ; "1408")]
#[test_case(r#####"tw`border-l-red-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(153 27 27 / var(--tw-border-opacity))",
})
;"##### ; "1409")]
#[test_case(r#####"tw`border-l-red-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(127 29 29 / var(--tw-border-opacity))",
})
;"##### ; "1410")]
#[test_case(r#####"tw`border-l-orange-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(255 247 237 / var(--tw-border-opacity))",
})
;"##### ; "1411")]
#[test_case(r#####"tw`border-l-orange-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(255 237 213 / var(--tw-border-opacity))",
})
;"##### ; "1412")]
#[test_case(r#####"tw`border-l-orange-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 215 170 / var(--tw-border-opacity))",
})
;"##### ; "1413")]
#[test_case(r#####"tw`border-l-orange-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(253 186 116 / var(--tw-border-opacity))",
})
;"##### ; "1414")]
#[test_case(r#####"tw`border-l-orange-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(251 146 60 / var(--tw-border-opacity))",
})
;"##### ; "1415")]
#[test_case(r#####"tw`border-l-orange-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(249 115 22 / var(--tw-border-opacity))",
})
;"##### ; "1416")]
#[test_case(r#####"tw`border-l-orange-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(234 88 12 / var(--tw-border-opacity))",
})
;"##### ; "1417")]
#[test_case(r#####"tw`border-l-orange-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(194 65 12 / var(--tw-border-opacity))",
})
;"##### ; "1418")]
#[test_case(r#####"tw`border-l-orange-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(154 52 18 / var(--tw-border-opacity))",
})
;"##### ; "1419")]
#[test_case(r#####"tw`border-l-orange-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(124 45 18 / var(--tw-border-opacity))",
})
;"##### ; "1420")]
#[test_case(r#####"tw`border-l-amber-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(255 251 235 / var(--tw-border-opacity))",
})
;"##### ; "1421")]
#[test_case(r#####"tw`border-l-amber-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 243 199 / var(--tw-border-opacity))",
})
;"##### ; "1422")]
#[test_case(r#####"tw`border-l-amber-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(253 230 138 / var(--tw-border-opacity))",
})
;"##### ; "1423")]
#[test_case(r#####"tw`border-l-amber-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(252 211 77 / var(--tw-border-opacity))",
})
;"##### ; "1424")]
#[test_case(r#####"tw`border-l-amber-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(251 191 36 / var(--tw-border-opacity))",
})
;"##### ; "1425")]
#[test_case(r#####"tw`border-l-amber-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(245 158 11 / var(--tw-border-opacity))",
})
;"##### ; "1426")]
#[test_case(r#####"tw`border-l-amber-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(217 119 6 / var(--tw-border-opacity))",
})
;"##### ; "1427")]
#[test_case(r#####"tw`border-l-amber-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(180 83 9 / var(--tw-border-opacity))",
})
;"##### ; "1428")]
#[test_case(r#####"tw`border-l-amber-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(146 64 14 / var(--tw-border-opacity))",
})
;"##### ; "1429")]
#[test_case(r#####"tw`border-l-amber-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(120 53 15 / var(--tw-border-opacity))",
})
;"##### ; "1430")]
#[test_case(r#####"tw`border-l-yellow-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 252 232 / var(--tw-border-opacity))",
})
;"##### ; "1431")]
#[test_case(r#####"tw`border-l-yellow-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 249 195 / var(--tw-border-opacity))",
})
;"##### ; "1432")]
#[test_case(r#####"tw`border-l-yellow-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 240 138 / var(--tw-border-opacity))",
})
;"##### ; "1433")]
#[test_case(r#####"tw`border-l-yellow-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(253 224 71 / var(--tw-border-opacity))",
})
;"##### ; "1434")]
#[test_case(r#####"tw`border-l-yellow-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(250 204 21 / var(--tw-border-opacity))",
})
;"##### ; "1435")]
#[test_case(r#####"tw`border-l-yellow-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(234 179 8 / var(--tw-border-opacity))",
})
;"##### ; "1436")]
#[test_case(r#####"tw`border-l-yellow-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(202 138 4 / var(--tw-border-opacity))",
})
;"##### ; "1437")]
#[test_case(r#####"tw`border-l-yellow-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(161 98 7 / var(--tw-border-opacity))",
})
;"##### ; "1438")]
#[test_case(r#####"tw`border-l-yellow-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(133 77 14 / var(--tw-border-opacity))",
})
;"##### ; "1439")]
#[test_case(r#####"tw`border-l-yellow-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(113 63 18 / var(--tw-border-opacity))",
})
;"##### ; "1440")]
#[test_case(r#####"tw`border-l-lime-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(247 254 231 / var(--tw-border-opacity))",
})
;"##### ; "1441")]
#[test_case(r#####"tw`border-l-lime-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(236 252 203 / var(--tw-border-opacity))",
})
;"##### ; "1442")]
#[test_case(r#####"tw`border-l-lime-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(217 249 157 / var(--tw-border-opacity))",
})
;"##### ; "1443")]
#[test_case(r#####"tw`border-l-lime-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(190 242 100 / var(--tw-border-opacity))",
})
;"##### ; "1444")]
#[test_case(r#####"tw`border-l-lime-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(163 230 53 / var(--tw-border-opacity))",
})
;"##### ; "1445")]
#[test_case(r#####"tw`border-l-lime-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(132 204 22 / var(--tw-border-opacity))",
})
;"##### ; "1446")]
#[test_case(r#####"tw`border-l-lime-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(101 163 13 / var(--tw-border-opacity))",
})
;"##### ; "1447")]
#[test_case(r#####"tw`border-l-lime-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(77 124 15 / var(--tw-border-opacity))",
})
;"##### ; "1448")]
#[test_case(r#####"tw`border-l-lime-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(63 98 18 / var(--tw-border-opacity))",
})
;"##### ; "1449")]
#[test_case(r#####"tw`border-l-lime-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(54 83 20 / var(--tw-border-opacity))",
})
;"##### ; "1450")]
#[test_case(r#####"tw`border-l-green-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(240 253 244 / var(--tw-border-opacity))",
})
;"##### ; "1451")]
#[test_case(r#####"tw`border-l-green-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(220 252 231 / var(--tw-border-opacity))",
})
;"##### ; "1452")]
#[test_case(r#####"tw`border-l-green-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(187 247 208 / var(--tw-border-opacity))",
})
;"##### ; "1453")]
#[test_case(r#####"tw`border-l-green-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(134 239 172 / var(--tw-border-opacity))",
})
;"##### ; "1454")]
#[test_case(r#####"tw`border-l-green-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(74 222 128 / var(--tw-border-opacity))",
})
;"##### ; "1455")]
#[test_case(r#####"tw`border-l-green-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(34 197 94 / var(--tw-border-opacity))",
})
;"##### ; "1456")]
#[test_case(r#####"tw`border-l-green-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(22 163 74 / var(--tw-border-opacity))",
})
;"##### ; "1457")]
#[test_case(r#####"tw`border-l-green-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(21 128 61 / var(--tw-border-opacity))",
})
;"##### ; "1458")]
#[test_case(r#####"tw`border-l-green-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(22 101 52 / var(--tw-border-opacity))",
})
;"##### ; "1459")]
#[test_case(r#####"tw`border-l-green-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(20 83 45 / var(--tw-border-opacity))",
})
;"##### ; "1460")]
#[test_case(r#####"tw`border-l-emerald-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(236 253 245 / var(--tw-border-opacity))",
})
;"##### ; "1461")]
#[test_case(r#####"tw`border-l-emerald-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(209 250 229 / var(--tw-border-opacity))",
})
;"##### ; "1462")]
#[test_case(r#####"tw`border-l-emerald-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(167 243 208 / var(--tw-border-opacity))",
})
;"##### ; "1463")]
#[test_case(r#####"tw`border-l-emerald-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(110 231 183 / var(--tw-border-opacity))",
})
;"##### ; "1464")]
#[test_case(r#####"tw`border-l-emerald-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(52 211 153 / var(--tw-border-opacity))",
})
;"##### ; "1465")]
#[test_case(r#####"tw`border-l-emerald-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(16 185 129 / var(--tw-border-opacity))",
})
;"##### ; "1466")]
#[test_case(r#####"tw`border-l-emerald-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(5 150 105 / var(--tw-border-opacity))",
})
;"##### ; "1467")]
#[test_case(r#####"tw`border-l-emerald-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(4 120 87 / var(--tw-border-opacity))",
})
;"##### ; "1468")]
#[test_case(r#####"tw`border-l-emerald-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(6 95 70 / var(--tw-border-opacity))",
})
;"##### ; "1469")]
#[test_case(r#####"tw`border-l-emerald-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(6 78 59 / var(--tw-border-opacity))",
})
;"##### ; "1470")]
#[test_case(r#####"tw`border-l-teal-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(240 253 250 / var(--tw-border-opacity))",
})
;"##### ; "1471")]
#[test_case(r#####"tw`border-l-teal-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(204 251 241 / var(--tw-border-opacity))",
})
;"##### ; "1472")]
#[test_case(r#####"tw`border-l-teal-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(153 246 228 / var(--tw-border-opacity))",
})
;"##### ; "1473")]
#[test_case(r#####"tw`border-l-teal-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(94 234 212 / var(--tw-border-opacity))",
})
;"##### ; "1474")]
#[test_case(r#####"tw`border-l-teal-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(45 212 191 / var(--tw-border-opacity))",
})
;"##### ; "1475")]
#[test_case(r#####"tw`border-l-teal-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(20 184 166 / var(--tw-border-opacity))",
})
;"##### ; "1476")]
#[test_case(r#####"tw`border-l-teal-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(13 148 136 / var(--tw-border-opacity))",
})
;"##### ; "1477")]
#[test_case(r#####"tw`border-l-teal-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(15 118 110 / var(--tw-border-opacity))",
})
;"##### ; "1478")]
#[test_case(r#####"tw`border-l-teal-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(17 94 89 / var(--tw-border-opacity))",
})
;"##### ; "1479")]
#[test_case(r#####"tw`border-l-teal-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(19 78 74 / var(--tw-border-opacity))",
})
;"##### ; "1480")]
#[test_case(r#####"tw`border-l-cyan-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(236 254 255 / var(--tw-border-opacity))",
})
;"##### ; "1481")]
#[test_case(r#####"tw`border-l-cyan-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(207 250 254 / var(--tw-border-opacity))",
})
;"##### ; "1482")]
#[test_case(r#####"tw`border-l-cyan-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(165 243 252 / var(--tw-border-opacity))",
})
;"##### ; "1483")]
#[test_case(r#####"tw`border-l-cyan-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(103 232 249 / var(--tw-border-opacity))",
})
;"##### ; "1484")]
#[test_case(r#####"tw`border-l-cyan-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(34 211 238 / var(--tw-border-opacity))",
})
;"##### ; "1485")]
#[test_case(r#####"tw`border-l-cyan-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(6 182 212 / var(--tw-border-opacity))",
})
;"##### ; "1486")]
#[test_case(r#####"tw`border-l-cyan-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(8 145 178 / var(--tw-border-opacity))",
})
;"##### ; "1487")]
#[test_case(r#####"tw`border-l-cyan-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(14 116 144 / var(--tw-border-opacity))",
})
;"##### ; "1488")]
#[test_case(r#####"tw`border-l-cyan-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(21 94 117 / var(--tw-border-opacity))",
})
;"##### ; "1489")]
#[test_case(r#####"tw`border-l-cyan-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(22 78 99 / var(--tw-border-opacity))",
})
;"##### ; "1490")]
#[test_case(r#####"tw`border-l-sky-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(240 249 255 / var(--tw-border-opacity))",
})
;"##### ; "1491")]
#[test_case(r#####"tw`border-l-sky-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(224 242 254 / var(--tw-border-opacity))",
})
;"##### ; "1492")]
#[test_case(r#####"tw`border-l-sky-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(186 230 253 / var(--tw-border-opacity))",
})
;"##### ; "1493")]
#[test_case(r#####"tw`border-l-sky-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(125 211 252 / var(--tw-border-opacity))",
})
;"##### ; "1494")]
#[test_case(r#####"tw`border-l-sky-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(56 189 248 / var(--tw-border-opacity))",
})
;"##### ; "1495")]
#[test_case(r#####"tw`border-l-sky-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(14 165 233 / var(--tw-border-opacity))",
})
;"##### ; "1496")]
#[test_case(r#####"tw`border-l-sky-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(2 132 199 / var(--tw-border-opacity))",
})
;"##### ; "1497")]
#[test_case(r#####"tw`border-l-sky-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(3 105 161 / var(--tw-border-opacity))",
})
;"##### ; "1498")]
#[test_case(r#####"tw`border-l-sky-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(7 89 133 / var(--tw-border-opacity))",
})
;"##### ; "1499")]
#[test_case(r#####"tw`border-l-sky-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(12 74 110 / var(--tw-border-opacity))",
})
;"##### ; "1500")]
#[test_case(r#####"tw`border-l-blue-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(239 246 255 / var(--tw-border-opacity))",
})
;"##### ; "1501")]
#[test_case(r#####"tw`border-l-blue-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(219 234 254 / var(--tw-border-opacity))",
})
;"##### ; "1502")]
#[test_case(r#####"tw`border-l-blue-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(191 219 254 / var(--tw-border-opacity))",
})
;"##### ; "1503")]
#[test_case(r#####"tw`border-l-blue-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(147 197 253 / var(--tw-border-opacity))",
})
;"##### ; "1504")]
#[test_case(r#####"tw`border-l-blue-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(96 165 250 / var(--tw-border-opacity))",
})
;"##### ; "1505")]
#[test_case(r#####"tw`border-l-blue-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(59 130 246 / var(--tw-border-opacity))",
})
;"##### ; "1506")]
#[test_case(r#####"tw`border-l-blue-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(37 99 235 / var(--tw-border-opacity))",
})
;"##### ; "1507")]
#[test_case(r#####"tw`border-l-blue-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(29 78 216 / var(--tw-border-opacity))",
})
;"##### ; "1508")]
#[test_case(r#####"tw`border-l-blue-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(30 64 175 / var(--tw-border-opacity))",
})
;"##### ; "1509")]
#[test_case(r#####"tw`border-l-blue-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(30 58 138 / var(--tw-border-opacity))",
})
;"##### ; "1510")]
#[test_case(r#####"tw`border-l-indigo-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(238 242 255 / var(--tw-border-opacity))",
})
;"##### ; "1511")]
#[test_case(r#####"tw`border-l-indigo-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(224 231 255 / var(--tw-border-opacity))",
})
;"##### ; "1512")]
#[test_case(r#####"tw`border-l-indigo-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(199 210 254 / var(--tw-border-opacity))",
})
;"##### ; "1513")]
#[test_case(r#####"tw`border-l-indigo-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(165 180 252 / var(--tw-border-opacity))",
})
;"##### ; "1514")]
#[test_case(r#####"tw`border-l-indigo-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(129 140 248 / var(--tw-border-opacity))",
})
;"##### ; "1515")]
#[test_case(r#####"tw`border-l-indigo-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(99 102 241 / var(--tw-border-opacity))",
})
;"##### ; "1516")]
#[test_case(r#####"tw`border-l-indigo-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(79 70 229 / var(--tw-border-opacity))",
})
;"##### ; "1517")]
#[test_case(r#####"tw`border-l-indigo-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(67 56 202 / var(--tw-border-opacity))",
})
;"##### ; "1518")]
#[test_case(r#####"tw`border-l-indigo-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(55 48 163 / var(--tw-border-opacity))",
})
;"##### ; "1519")]
#[test_case(r#####"tw`border-l-indigo-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(49 46 129 / var(--tw-border-opacity))",
})
;"##### ; "1520")]
#[test_case(r#####"tw`border-l-violet-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(245 243 255 / var(--tw-border-opacity))",
})
;"##### ; "1521")]
#[test_case(r#####"tw`border-l-violet-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(237 233 254 / var(--tw-border-opacity))",
})
;"##### ; "1522")]
#[test_case(r#####"tw`border-l-violet-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(221 214 254 / var(--tw-border-opacity))",
})
;"##### ; "1523")]
#[test_case(r#####"tw`border-l-violet-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(196 181 253 / var(--tw-border-opacity))",
})
;"##### ; "1524")]
#[test_case(r#####"tw`border-l-violet-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(167 139 250 / var(--tw-border-opacity))",
})
;"##### ; "1525")]
#[test_case(r#####"tw`border-l-violet-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(139 92 246 / var(--tw-border-opacity))",
})
;"##### ; "1526")]
#[test_case(r#####"tw`border-l-violet-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(124 58 237 / var(--tw-border-opacity))",
})
;"##### ; "1527")]
#[test_case(r#####"tw`border-l-violet-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(109 40 217 / var(--tw-border-opacity))",
})
;"##### ; "1528")]
#[test_case(r#####"tw`border-l-violet-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(91 33 182 / var(--tw-border-opacity))",
})
;"##### ; "1529")]
#[test_case(r#####"tw`border-l-violet-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(76 29 149 / var(--tw-border-opacity))",
})
;"##### ; "1530")]
#[test_case(r#####"tw`border-l-purple-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(250 245 255 / var(--tw-border-opacity))",
})
;"##### ; "1531")]
#[test_case(r#####"tw`border-l-purple-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(243 232 255 / var(--tw-border-opacity))",
})
;"##### ; "1532")]
#[test_case(r#####"tw`border-l-purple-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(233 213 255 / var(--tw-border-opacity))",
})
;"##### ; "1533")]
#[test_case(r#####"tw`border-l-purple-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(216 180 254 / var(--tw-border-opacity))",
})
;"##### ; "1534")]
#[test_case(r#####"tw`border-l-purple-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(192 132 252 / var(--tw-border-opacity))",
})
;"##### ; "1535")]
#[test_case(r#####"tw`border-l-purple-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(168 85 247 / var(--tw-border-opacity))",
})
;"##### ; "1536")]
#[test_case(r#####"tw`border-l-purple-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(147 51 234 / var(--tw-border-opacity))",
})
;"##### ; "1537")]
#[test_case(r#####"tw`border-l-purple-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(126 34 206 / var(--tw-border-opacity))",
})
;"##### ; "1538")]
#[test_case(r#####"tw`border-l-purple-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(107 33 168 / var(--tw-border-opacity))",
})
;"##### ; "1539")]
#[test_case(r#####"tw`border-l-purple-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(88 28 135 / var(--tw-border-opacity))",
})
;"##### ; "1540")]
#[test_case(r#####"tw`border-l-fuchsia-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(253 244 255 / var(--tw-border-opacity))",
})
;"##### ; "1541")]
#[test_case(r#####"tw`border-l-fuchsia-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(250 232 255 / var(--tw-border-opacity))",
})
;"##### ; "1542")]
#[test_case(r#####"tw`border-l-fuchsia-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(245 208 254 / var(--tw-border-opacity))",
})
;"##### ; "1543")]
#[test_case(r#####"tw`border-l-fuchsia-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(240 171 252 / var(--tw-border-opacity))",
})
;"##### ; "1544")]
#[test_case(r#####"tw`border-l-fuchsia-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(232 121 249 / var(--tw-border-opacity))",
})
;"##### ; "1545")]
#[test_case(r#####"tw`border-l-fuchsia-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(217 70 239 / var(--tw-border-opacity))",
})
;"##### ; "1546")]
#[test_case(r#####"tw`border-l-fuchsia-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(192 38 211 / var(--tw-border-opacity))",
})
;"##### ; "1547")]
#[test_case(r#####"tw`border-l-fuchsia-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(162 28 175 / var(--tw-border-opacity))",
})
;"##### ; "1548")]
#[test_case(r#####"tw`border-l-fuchsia-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(134 25 143 / var(--tw-border-opacity))",
})
;"##### ; "1549")]
#[test_case(r#####"tw`border-l-fuchsia-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(112 26 117 / var(--tw-border-opacity))",
})
;"##### ; "1550")]
#[test_case(r#####"tw`border-l-pink-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(253 242 248 / var(--tw-border-opacity))",
})
;"##### ; "1551")]
#[test_case(r#####"tw`border-l-pink-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(252 231 243 / var(--tw-border-opacity))",
})
;"##### ; "1552")]
#[test_case(r#####"tw`border-l-pink-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(251 207 232 / var(--tw-border-opacity))",
})
;"##### ; "1553")]
#[test_case(r#####"tw`border-l-pink-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(249 168 212 / var(--tw-border-opacity))",
})
;"##### ; "1554")]
#[test_case(r#####"tw`border-l-pink-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(244 114 182 / var(--tw-border-opacity))",
})
;"##### ; "1555")]
#[test_case(r#####"tw`border-l-pink-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(236 72 153 / var(--tw-border-opacity))",
})
;"##### ; "1556")]
#[test_case(r#####"tw`border-l-pink-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(219 39 119 / var(--tw-border-opacity))",
})
;"##### ; "1557")]
#[test_case(r#####"tw`border-l-pink-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(190 24 93 / var(--tw-border-opacity))",
})
;"##### ; "1558")]
#[test_case(r#####"tw`border-l-pink-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(157 23 77 / var(--tw-border-opacity))",
})
;"##### ; "1559")]
#[test_case(r#####"tw`border-l-pink-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(131 24 67 / var(--tw-border-opacity))",
})
;"##### ; "1560")]
#[test_case(r#####"tw`border-l-rose-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(255 241 242 / var(--tw-border-opacity))",
})
;"##### ; "1561")]
#[test_case(r#####"tw`border-l-rose-100`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(255 228 230 / var(--tw-border-opacity))",
})
;"##### ; "1562")]
#[test_case(r#####"tw`border-l-rose-200`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(254 205 211 / var(--tw-border-opacity))",
})
;"##### ; "1563")]
#[test_case(r#####"tw`border-l-rose-300`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(253 164 175 / var(--tw-border-opacity))",
})
;"##### ; "1564")]
#[test_case(r#####"tw`border-l-rose-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(251 113 133 / var(--tw-border-opacity))",
})
;"##### ; "1565")]
#[test_case(r#####"tw`border-l-rose-500`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(244 63 94 / var(--tw-border-opacity))",
})
;"##### ; "1566")]
#[test_case(r#####"tw`border-l-rose-600`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(225 29 72 / var(--tw-border-opacity))",
})
;"##### ; "1567")]
#[test_case(r#####"tw`border-l-rose-700`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(190 18 60 / var(--tw-border-opacity))",
})
;"##### ; "1568")]
#[test_case(r#####"tw`border-l-rose-800`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(159 18 57 / var(--tw-border-opacity))",
})
;"##### ; "1569")]
#[test_case(r#####"tw`border-l-rose-900`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(136 19 55 / var(--tw-border-opacity))",
})
;"##### ; "1570")]
#[test_case(r#####"tw`border-4 border-indigo-500/100`"#####, r#####"({
  borderWidth: "4px",
  borderColor: "rgb(99 102 241 / 1)",
})
;"##### ; "1571")]
#[test_case(r#####"tw`border-4 border-indigo-500/75`"#####, r#####"({
  borderWidth: "4px",
  borderColor: "rgb(99 102 241 / 0.75)",
})
;"##### ; "1572")]
#[test_case(r#####"tw`border-4 border-indigo-500/50`"#####, r#####"({
  borderWidth: "4px",
  borderColor: "rgb(99 102 241 / 0.5)",
})
;"##### ; "1573")]
#[test_case(r#####"tw`border-4 border-indigo-600/[.55]`"#####, r#####"({
  borderWidth: "4px",
  borderColor: "rgb(79 70 229 / .55)",
})
;"##### ; "1574")]
#[test_case(r#####"tw`border-[#243c5a]`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(36 60 90 / var(--tw-border-opacity))",
})
;"##### ; "1575")]
#[test_case(r#####"tw`border-[#f00]`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(255 0 0 / var(--tw-border-opacity))",
})
;"##### ; "1576")]
#[test_case(r#####"tw`border-t-[#f00]`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(255 0 0 / var(--tw-border-opacity))",
})
;"##### ; "1577")]
#[test_case(r#####"tw`border-red-500/25`"#####, r#####"({
  borderColor: "rgb(239 68 68 / 0.25)",
})
;"##### ; "1578")]
#[test_case(r#####"tw`border-red-500/fromConfig`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(0 0 0 / var(--tw-border-opacity))",
})
;"##### ; "1579")]
#[test_case(r#####"tw`border-red-500/fromConfig/25`"#####, r#####"({
  borderColor: "rgb(0 0 0 / 0.25)",
})
;"##### ; "1580")]
#[test_case(r#####"tw`border-red-500/fromConfig/[.555]`"#####, r#####"({
  borderColor: "rgb(0 0 0 / .555)",
})
;"##### ; "1581")]
#[test_case(r#####"tw`border-red-500/fromConfig/[var(--myvar)]`"#####, r#####"({
  borderColor: "rgb(0 0 0 / var(--myvar))",
})
;"##### ; "1582")]
#[test_case(r#####"tw`border-red-500/[.555]`"#####, r#####"({
  borderColor: "rgb(239 68 68 / .555)",
})
;"##### ; "1583")]
#[test_case(r#####"tw`border-red-500/[var(--myvar)]`"#####, r#####"({
  borderColor: "rgb(239 68 68 / var(--myvar))",
})
;"##### ; "1584")]
#[test_case(r#####"tw`border-[theme('colors.red.500')]`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(239 68 68 / var(--tw-border-opacity))",
})
;"##### ; "1585")]
#[test_case(r#####"tw`border-[theme('colors.red.500')]/20`"#####, r#####"({
  borderColor: "rgb(239 68 68 / 0.2)",
})
;"##### ; "1586")]
#[test_case(r#####"tw`border-electric/25`"#####, r#####"({
  borderColor: "rgba(219, 0, 255, 0.25)",
})
;"##### ; "1587")]
#[test_case(r#####"tw`border-electric/[.555]`"#####, r#####"({
  borderColor: "rgba(219, 0, 255, .555)",
})
;"##### ; "1588")]
#[test_case(r#####"tw`border-electric/[var(--myvar)]`"#####, r#####"({
  borderColor: "rgba(219, 0, 255, var(--myvar))",
})
;"##### ; "1589")]
#[test_case(r#####"tw`border-[theme('colors.electric')]`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(219 0 255 / var(--tw-border-opacity))",
})
;"##### ; "1590")]
#[test_case(r#####"tw`border-[theme('colors.electric')]/20`"#####, r#####"({
  borderColor: "rgb(219 0 255 / 0.2)",
})
;"##### ; "1591")]
#[test_case(r#####"tw`border-[hsla(235, 100%, 50%, .5)]`"#####, r#####"({
  borderColor: "hsla(235, 100%, 50%, .5)",
})
;"##### ; "1592")]
#[test_case(r#####"tw`border-[rgba(255, 255, 255, 0)]`"#####, r#####"({
  borderColor: "rgba(255, 255, 255, 0)",
})
;"##### ; "1593")]
#[test_case(r#####"tw`border-[red_black_white rgb(255, 255,255,0)]`"#####, r#####"({
  borderColor: "red black white rgb(255, 255,255,0)",
})
;"##### ; "1594")]
#[test_case(r#####"tw`border-[red black_blue]`"#####, r#####"({
  borderColor: "red black blue",
})
;"##### ; "1595")]
#[test_case(r#####"tw`border-[red black]`"#####, r#####"({
  borderColor: "red black",
})
;"##### ; "1596")]
#[test_case(r#####"tw`border-[hsl(50 50% 50%)]`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "hsl(50 50% 50% / var(--tw-border-opacity))",
})
;"##### ; "1597")]
#[test_case(r#####"tw`border-t-[color:green]`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(0 128 0 / var(--tw-border-opacity))",
})
;"##### ; "1598")]
#[test_case(r#####"tw`border-t-[color:rgba(255, 255, 255, .45)]`"#####, r#####"({
  borderTopColor: "rgba(255, 255, 255, .45)",
})
;"##### ; "1599")]
#[test_case(r#####"tw`border-r-[color:green]`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(0 128 0 / var(--tw-border-opacity))",
})
;"##### ; "1600")]
#[test_case(r#####"tw`border-r-[color:rgba(255, 255, 255, .45)]`"#####, r#####"({
  borderRightColor: "rgba(255, 255, 255, .45)",
})
;"##### ; "1601")]
#[test_case(r#####"tw`border-b-[color:green]`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(0 128 0 / var(--tw-border-opacity))",
})
;"##### ; "1602")]
#[test_case(r#####"tw`border-b-[color:rgba(255, 255, 255, .45)]`"#####, r#####"({
  borderBottomColor: "rgba(255, 255, 255, .45)",
})
;"##### ; "1603")]
#[test_case(r#####"tw`border-l-[color:green]`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(0 128 0 / var(--tw-border-opacity))",
})
;"##### ; "1604")]
#[test_case(r#####"tw`border-l-[color:rgba(255, 255, 255, .45)]`"#####, r#####"({
  borderLeftColor: "rgba(255, 255, 255, .45)",
})
;"##### ; "1605")]
#[test_case(r#####"tw`border-[color:green]`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(0 128 0 / var(--tw-border-opacity))",
})
;"##### ; "1606")]
#[test_case(r#####"tw`border-[color:rgba(255, 255, 255, .45)]`"#####, r#####"({
  borderColor: "rgba(255, 255, 255, .45)",
})
;"##### ; "1607")]
#[test_case(r#####"tw`border-x-[color:green]`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(0 128 0 / var(--tw-border-opacity))",
  borderRightColor: "rgb(0 128 0 / var(--tw-border-opacity))",
})
;"##### ; "1608")]
#[test_case(r#####"tw`border-y-[color:rgba(255, 255, 255, .45)]`"#####, r#####"({
  borderTopColor: "rgba(255, 255, 255, .45)",
  borderBottomColor: "rgba(255, 255, 255, .45)",
})
;"##### ; "1609")]
#[test_case(r#####"tw`border-black border-s-green-500 border-e-red-400`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderColor: "rgb(0 0 0 / var(--tw-border-opacity))",
  borderInlineEndColor: "rgb(248 113 113 / var(--tw-border-opacity))",
  borderInlineStartColor: "rgb(34 197 94 / var(--tw-border-opacity))",
})"##### ; "1610")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
