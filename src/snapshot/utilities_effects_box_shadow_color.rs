use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`boxShadowColor`"#####, r#####"({
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
})
;"##### ; "0")]
#[test_case(r#####"tw`shadow-inherit`"#####, r#####"({
  "--tw-shadow-color": "inherit",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "1")]
#[test_case(r#####"tw`shadow-current`"#####, r#####"({
  "--tw-shadow-color": "currentColor",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "2")]
#[test_case(r#####"tw`shadow-transparent`"#####, r#####"({
  "--tw-shadow-color": "transparent",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "3")]
#[test_case(r#####"tw`shadow-black`"#####, r#####"({
  "--tw-shadow-color": "#000",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "4")]
#[test_case(r#####"tw`shadow-white`"#####, r#####"({
  "--tw-shadow-color": "#fff",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "5")]
#[test_case(r#####"tw`shadow-slate-50`"#####, r#####"({
  "--tw-shadow-color": "#f8fafc",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "6")]
#[test_case(r#####"tw`shadow-slate-100`"#####, r#####"({
  "--tw-shadow-color": "#f1f5f9",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "7")]
#[test_case(r#####"tw`shadow-slate-200`"#####, r#####"({
  "--tw-shadow-color": "#e2e8f0",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "8")]
#[test_case(r#####"tw`shadow-slate-300`"#####, r#####"({
  "--tw-shadow-color": "#cbd5e1",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "9")]
#[test_case(r#####"tw`shadow-slate-400`"#####, r#####"({
  "--tw-shadow-color": "#94a3b8",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "10")]
#[test_case(r#####"tw`shadow-slate-500`"#####, r#####"({
  "--tw-shadow-color": "#64748b",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "11")]
#[test_case(r#####"tw`shadow-slate-600`"#####, r#####"({
  "--tw-shadow-color": "#475569",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "12")]
#[test_case(r#####"tw`shadow-slate-700`"#####, r#####"({
  "--tw-shadow-color": "#334155",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "13")]
#[test_case(r#####"tw`shadow-slate-800`"#####, r#####"({
  "--tw-shadow-color": "#1e293b",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "14")]
#[test_case(r#####"tw`shadow-slate-900`"#####, r#####"({
  "--tw-shadow-color": "#0f172a",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "15")]
#[test_case(r#####"tw`shadow-gray-50`"#####, r#####"({
  "--tw-shadow-color": "#f9fafb",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "16")]
#[test_case(r#####"tw`shadow-gray-100`"#####, r#####"({
  "--tw-shadow-color": "#f3f4f6",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "17")]
#[test_case(r#####"tw`shadow-gray-200`"#####, r#####"({
  "--tw-shadow-color": "#e5e7eb",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "18")]
#[test_case(r#####"tw`shadow-gray-300`"#####, r#####"({
  "--tw-shadow-color": "#d1d5db",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "19")]
#[test_case(r#####"tw`shadow-gray-400`"#####, r#####"({
  "--tw-shadow-color": "#9ca3af",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "20")]
#[test_case(r#####"tw`shadow-gray-500`"#####, r#####"({
  "--tw-shadow-color": "#6b7280",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "21")]
#[test_case(r#####"tw`shadow-gray-600`"#####, r#####"({
  "--tw-shadow-color": "#4b5563",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "22")]
#[test_case(r#####"tw`shadow-gray-700`"#####, r#####"({
  "--tw-shadow-color": "#374151",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "23")]
#[test_case(r#####"tw`shadow-gray-800`"#####, r#####"({
  "--tw-shadow-color": "#1f2937",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "24")]
#[test_case(r#####"tw`shadow-gray-900`"#####, r#####"({
  "--tw-shadow-color": "#111827",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "25")]
#[test_case(r#####"tw`shadow-zinc-50`"#####, r#####"({
  "--tw-shadow-color": "#fafafa",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "26")]
#[test_case(r#####"tw`shadow-zinc-100`"#####, r#####"({
  "--tw-shadow-color": "#f4f4f5",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "27")]
#[test_case(r#####"tw`shadow-zinc-200`"#####, r#####"({
  "--tw-shadow-color": "#e4e4e7",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "28")]
#[test_case(r#####"tw`shadow-zinc-300`"#####, r#####"({
  "--tw-shadow-color": "#d4d4d8",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "29")]
#[test_case(r#####"tw`shadow-zinc-400`"#####, r#####"({
  "--tw-shadow-color": "#a1a1aa",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "30")]
#[test_case(r#####"tw`shadow-zinc-500`"#####, r#####"({
  "--tw-shadow-color": "#71717a",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "31")]
#[test_case(r#####"tw`shadow-zinc-600`"#####, r#####"({
  "--tw-shadow-color": "#52525b",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "32")]
#[test_case(r#####"tw`shadow-zinc-700`"#####, r#####"({
  "--tw-shadow-color": "#3f3f46",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "33")]
#[test_case(r#####"tw`shadow-zinc-800`"#####, r#####"({
  "--tw-shadow-color": "#27272a",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "34")]
#[test_case(r#####"tw`shadow-zinc-900`"#####, r#####"({
  "--tw-shadow-color": "#18181b",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "35")]
#[test_case(r#####"tw`shadow-neutral-50`"#####, r#####"({
  "--tw-shadow-color": "#fafafa",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "36")]
#[test_case(r#####"tw`shadow-neutral-100`"#####, r#####"({
  "--tw-shadow-color": "#f5f5f5",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "37")]
#[test_case(r#####"tw`shadow-neutral-200`"#####, r#####"({
  "--tw-shadow-color": "#e5e5e5",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "38")]
#[test_case(r#####"tw`shadow-neutral-300`"#####, r#####"({
  "--tw-shadow-color": "#d4d4d4",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "39")]
#[test_case(r#####"tw`shadow-neutral-400`"#####, r#####"({
  "--tw-shadow-color": "#a3a3a3",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "40")]
#[test_case(r#####"tw`shadow-neutral-500`"#####, r#####"({
  "--tw-shadow-color": "#737373",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "41")]
#[test_case(r#####"tw`shadow-neutral-600`"#####, r#####"({
  "--tw-shadow-color": "#525252",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "42")]
#[test_case(r#####"tw`shadow-neutral-700`"#####, r#####"({
  "--tw-shadow-color": "#404040",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "43")]
#[test_case(r#####"tw`shadow-neutral-800`"#####, r#####"({
  "--tw-shadow-color": "#262626",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "44")]
#[test_case(r#####"tw`shadow-neutral-900`"#####, r#####"({
  "--tw-shadow-color": "#171717",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "45")]
#[test_case(r#####"tw`shadow-stone-50`"#####, r#####"({
  "--tw-shadow-color": "#fafaf9",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "46")]
#[test_case(r#####"tw`shadow-stone-100`"#####, r#####"({
  "--tw-shadow-color": "#f5f5f4",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "47")]
#[test_case(r#####"tw`shadow-stone-200`"#####, r#####"({
  "--tw-shadow-color": "#e7e5e4",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "48")]
#[test_case(r#####"tw`shadow-stone-300`"#####, r#####"({
  "--tw-shadow-color": "#d6d3d1",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "49")]
#[test_case(r#####"tw`shadow-stone-400`"#####, r#####"({
  "--tw-shadow-color": "#a8a29e",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "50")]
#[test_case(r#####"tw`shadow-stone-500`"#####, r#####"({
  "--tw-shadow-color": "#78716c",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "51")]
#[test_case(r#####"tw`shadow-stone-600`"#####, r#####"({
  "--tw-shadow-color": "#57534e",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "52")]
#[test_case(r#####"tw`shadow-stone-700`"#####, r#####"({
  "--tw-shadow-color": "#44403c",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "53")]
#[test_case(r#####"tw`shadow-stone-800`"#####, r#####"({
  "--tw-shadow-color": "#292524",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "54")]
#[test_case(r#####"tw`shadow-stone-900`"#####, r#####"({
  "--tw-shadow-color": "#1c1917",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "55")]
#[test_case(r#####"tw`shadow-red-50`"#####, r#####"({
  "--tw-shadow-color": "#fef2f2",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "56")]
#[test_case(r#####"tw`shadow-red-100`"#####, r#####"({
  "--tw-shadow-color": "#fee2e2",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "57")]
#[test_case(r#####"tw`shadow-red-200`"#####, r#####"({
  "--tw-shadow-color": "#fecaca",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "58")]
#[test_case(r#####"tw`shadow-red-300`"#####, r#####"({
  "--tw-shadow-color": "#fca5a5",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "59")]
#[test_case(r#####"tw`shadow-red-400`"#####, r#####"({
  "--tw-shadow-color": "#f87171",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "60")]
#[test_case(r#####"tw`shadow-red-500`"#####, r#####"({
  "--tw-shadow-color": "#ef4444",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "61")]
#[test_case(r#####"tw`shadow-red-600`"#####, r#####"({
  "--tw-shadow-color": "#dc2626",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "62")]
#[test_case(r#####"tw`shadow-red-700`"#####, r#####"({
  "--tw-shadow-color": "#b91c1c",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "63")]
#[test_case(r#####"tw`shadow-red-800`"#####, r#####"({
  "--tw-shadow-color": "#991b1b",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "64")]
#[test_case(r#####"tw`shadow-red-900`"#####, r#####"({
  "--tw-shadow-color": "#7f1d1d",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "65")]
#[test_case(r#####"tw`shadow-orange-50`"#####, r#####"({
  "--tw-shadow-color": "#fff7ed",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "66")]
#[test_case(r#####"tw`shadow-orange-100`"#####, r#####"({
  "--tw-shadow-color": "#ffedd5",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "67")]
#[test_case(r#####"tw`shadow-orange-200`"#####, r#####"({
  "--tw-shadow-color": "#fed7aa",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "68")]
#[test_case(r#####"tw`shadow-orange-300`"#####, r#####"({
  "--tw-shadow-color": "#fdba74",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "69")]
#[test_case(r#####"tw`shadow-orange-400`"#####, r#####"({
  "--tw-shadow-color": "#fb923c",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "70")]
#[test_case(r#####"tw`shadow-orange-500`"#####, r#####"({
  "--tw-shadow-color": "#f97316",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "71")]
#[test_case(r#####"tw`shadow-orange-600`"#####, r#####"({
  "--tw-shadow-color": "#ea580c",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "72")]
#[test_case(r#####"tw`shadow-orange-700`"#####, r#####"({
  "--tw-shadow-color": "#c2410c",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "73")]
#[test_case(r#####"tw`shadow-orange-800`"#####, r#####"({
  "--tw-shadow-color": "#9a3412",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "74")]
#[test_case(r#####"tw`shadow-orange-900`"#####, r#####"({
  "--tw-shadow-color": "#7c2d12",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "75")]
#[test_case(r#####"tw`shadow-amber-50`"#####, r#####"({
  "--tw-shadow-color": "#fffbeb",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "76")]
#[test_case(r#####"tw`shadow-amber-100`"#####, r#####"({
  "--tw-shadow-color": "#fef3c7",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "77")]
#[test_case(r#####"tw`shadow-amber-200`"#####, r#####"({
  "--tw-shadow-color": "#fde68a",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "78")]
#[test_case(r#####"tw`shadow-amber-300`"#####, r#####"({
  "--tw-shadow-color": "#fcd34d",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "79")]
#[test_case(r#####"tw`shadow-amber-400`"#####, r#####"({
  "--tw-shadow-color": "#fbbf24",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "80")]
#[test_case(r#####"tw`shadow-amber-500`"#####, r#####"({
  "--tw-shadow-color": "#f59e0b",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "81")]
#[test_case(r#####"tw`shadow-amber-600`"#####, r#####"({
  "--tw-shadow-color": "#d97706",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "82")]
#[test_case(r#####"tw`shadow-amber-700`"#####, r#####"({
  "--tw-shadow-color": "#b45309",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "83")]
#[test_case(r#####"tw`shadow-amber-800`"#####, r#####"({
  "--tw-shadow-color": "#92400e",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "84")]
#[test_case(r#####"tw`shadow-amber-900`"#####, r#####"({
  "--tw-shadow-color": "#78350f",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "85")]
#[test_case(r#####"tw`shadow-yellow-50`"#####, r#####"({
  "--tw-shadow-color": "#fefce8",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "86")]
#[test_case(r#####"tw`shadow-yellow-100`"#####, r#####"({
  "--tw-shadow-color": "#fef9c3",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "87")]
#[test_case(r#####"tw`shadow-yellow-200`"#####, r#####"({
  "--tw-shadow-color": "#fef08a",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "88")]
#[test_case(r#####"tw`shadow-yellow-300`"#####, r#####"({
  "--tw-shadow-color": "#fde047",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "89")]
#[test_case(r#####"tw`shadow-yellow-400`"#####, r#####"({
  "--tw-shadow-color": "#facc15",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "90")]
#[test_case(r#####"tw`shadow-yellow-500`"#####, r#####"({
  "--tw-shadow-color": "#eab308",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "91")]
#[test_case(r#####"tw`shadow-yellow-600`"#####, r#####"({
  "--tw-shadow-color": "#ca8a04",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "92")]
#[test_case(r#####"tw`shadow-yellow-700`"#####, r#####"({
  "--tw-shadow-color": "#a16207",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "93")]
#[test_case(r#####"tw`shadow-yellow-800`"#####, r#####"({
  "--tw-shadow-color": "#854d0e",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "94")]
#[test_case(r#####"tw`shadow-yellow-900`"#####, r#####"({
  "--tw-shadow-color": "#713f12",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "95")]
#[test_case(r#####"tw`shadow-lime-50`"#####, r#####"({
  "--tw-shadow-color": "#f7fee7",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "96")]
#[test_case(r#####"tw`shadow-lime-100`"#####, r#####"({
  "--tw-shadow-color": "#ecfccb",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "97")]
#[test_case(r#####"tw`shadow-lime-200`"#####, r#####"({
  "--tw-shadow-color": "#d9f99d",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "98")]
#[test_case(r#####"tw`shadow-lime-300`"#####, r#####"({
  "--tw-shadow-color": "#bef264",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "99")]
#[test_case(r#####"tw`shadow-lime-400`"#####, r#####"({
  "--tw-shadow-color": "#a3e635",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "100")]
#[test_case(r#####"tw`shadow-lime-500`"#####, r#####"({
  "--tw-shadow-color": "#84cc16",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "101")]
#[test_case(r#####"tw`shadow-lime-600`"#####, r#####"({
  "--tw-shadow-color": "#65a30d",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "102")]
#[test_case(r#####"tw`shadow-lime-700`"#####, r#####"({
  "--tw-shadow-color": "#4d7c0f",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "103")]
#[test_case(r#####"tw`shadow-lime-800`"#####, r#####"({
  "--tw-shadow-color": "#3f6212",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "104")]
#[test_case(r#####"tw`shadow-lime-900`"#####, r#####"({
  "--tw-shadow-color": "#365314",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "105")]
#[test_case(r#####"tw`shadow-green-50`"#####, r#####"({
  "--tw-shadow-color": "#f0fdf4",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "106")]
#[test_case(r#####"tw`shadow-green-100`"#####, r#####"({
  "--tw-shadow-color": "#dcfce7",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "107")]
#[test_case(r#####"tw`shadow-green-200`"#####, r#####"({
  "--tw-shadow-color": "#bbf7d0",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "108")]
#[test_case(r#####"tw`shadow-green-300`"#####, r#####"({
  "--tw-shadow-color": "#86efac",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "109")]
#[test_case(r#####"tw`shadow-green-400`"#####, r#####"({
  "--tw-shadow-color": "#4ade80",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "110")]
#[test_case(r#####"tw`shadow-green-500`"#####, r#####"({
  "--tw-shadow-color": "#22c55e",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "111")]
#[test_case(r#####"tw`shadow-green-600`"#####, r#####"({
  "--tw-shadow-color": "#16a34a",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "112")]
#[test_case(r#####"tw`shadow-green-700`"#####, r#####"({
  "--tw-shadow-color": "#15803d",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "113")]
#[test_case(r#####"tw`shadow-green-800`"#####, r#####"({
  "--tw-shadow-color": "#166534",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "114")]
#[test_case(r#####"tw`shadow-green-900`"#####, r#####"({
  "--tw-shadow-color": "#14532d",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "115")]
#[test_case(r#####"tw`shadow-emerald-50`"#####, r#####"({
  "--tw-shadow-color": "#ecfdf5",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "116")]
#[test_case(r#####"tw`shadow-emerald-100`"#####, r#####"({
  "--tw-shadow-color": "#d1fae5",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "117")]
#[test_case(r#####"tw`shadow-emerald-200`"#####, r#####"({
  "--tw-shadow-color": "#a7f3d0",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "118")]
#[test_case(r#####"tw`shadow-emerald-300`"#####, r#####"({
  "--tw-shadow-color": "#6ee7b7",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "119")]
#[test_case(r#####"tw`shadow-emerald-400`"#####, r#####"({
  "--tw-shadow-color": "#34d399",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "120")]
#[test_case(r#####"tw`shadow-emerald-500`"#####, r#####"({
  "--tw-shadow-color": "#10b981",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "121")]
#[test_case(r#####"tw`shadow-emerald-600`"#####, r#####"({
  "--tw-shadow-color": "#059669",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "122")]
#[test_case(r#####"tw`shadow-emerald-700`"#####, r#####"({
  "--tw-shadow-color": "#047857",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "123")]
#[test_case(r#####"tw`shadow-emerald-800`"#####, r#####"({
  "--tw-shadow-color": "#065f46",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "124")]
#[test_case(r#####"tw`shadow-emerald-900`"#####, r#####"({
  "--tw-shadow-color": "#064e3b",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "125")]
#[test_case(r#####"tw`shadow-teal-50`"#####, r#####"({
  "--tw-shadow-color": "#f0fdfa",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "126")]
#[test_case(r#####"tw`shadow-teal-100`"#####, r#####"({
  "--tw-shadow-color": "#ccfbf1",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "127")]
#[test_case(r#####"tw`shadow-teal-200`"#####, r#####"({
  "--tw-shadow-color": "#99f6e4",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "128")]
#[test_case(r#####"tw`shadow-teal-300`"#####, r#####"({
  "--tw-shadow-color": "#5eead4",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "129")]
#[test_case(r#####"tw`shadow-teal-400`"#####, r#####"({
  "--tw-shadow-color": "#2dd4bf",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "130")]
#[test_case(r#####"tw`shadow-teal-500`"#####, r#####"({
  "--tw-shadow-color": "#14b8a6",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "131")]
#[test_case(r#####"tw`shadow-teal-600`"#####, r#####"({
  "--tw-shadow-color": "#0d9488",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "132")]
#[test_case(r#####"tw`shadow-teal-700`"#####, r#####"({
  "--tw-shadow-color": "#0f766e",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "133")]
#[test_case(r#####"tw`shadow-teal-800`"#####, r#####"({
  "--tw-shadow-color": "#115e59",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "134")]
#[test_case(r#####"tw`shadow-teal-900`"#####, r#####"({
  "--tw-shadow-color": "#134e4a",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "135")]
#[test_case(r#####"tw`shadow-cyan-50`"#####, r#####"({
  "--tw-shadow-color": "#ecfeff",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "136")]
#[test_case(r#####"tw`shadow-cyan-100`"#####, r#####"({
  "--tw-shadow-color": "#cffafe",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "137")]
#[test_case(r#####"tw`shadow-cyan-200`"#####, r#####"({
  "--tw-shadow-color": "#a5f3fc",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "138")]
#[test_case(r#####"tw`shadow-cyan-300`"#####, r#####"({
  "--tw-shadow-color": "#67e8f9",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "139")]
#[test_case(r#####"tw`shadow-cyan-400`"#####, r#####"({
  "--tw-shadow-color": "#22d3ee",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "140")]
#[test_case(r#####"tw`shadow-cyan-500`"#####, r#####"({
  "--tw-shadow-color": "#06b6d4",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "141")]
#[test_case(r#####"tw`shadow-cyan-600`"#####, r#####"({
  "--tw-shadow-color": "#0891b2",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "142")]
#[test_case(r#####"tw`shadow-cyan-700`"#####, r#####"({
  "--tw-shadow-color": "#0e7490",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "143")]
#[test_case(r#####"tw`shadow-cyan-800`"#####, r#####"({
  "--tw-shadow-color": "#155e75",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "144")]
#[test_case(r#####"tw`shadow-cyan-900`"#####, r#####"({
  "--tw-shadow-color": "#164e63",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "145")]
#[test_case(r#####"tw`shadow-sky-50`"#####, r#####"({
  "--tw-shadow-color": "#f0f9ff",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "146")]
#[test_case(r#####"tw`shadow-sky-100`"#####, r#####"({
  "--tw-shadow-color": "#e0f2fe",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "147")]
#[test_case(r#####"tw`shadow-sky-200`"#####, r#####"({
  "--tw-shadow-color": "#bae6fd",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "148")]
#[test_case(r#####"tw`shadow-sky-300`"#####, r#####"({
  "--tw-shadow-color": "#7dd3fc",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "149")]
#[test_case(r#####"tw`shadow-sky-400`"#####, r#####"({
  "--tw-shadow-color": "#38bdf8",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "150")]
#[test_case(r#####"tw`shadow-sky-500`"#####, r#####"({
  "--tw-shadow-color": "#0ea5e9",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "151")]
#[test_case(r#####"tw`shadow-sky-600`"#####, r#####"({
  "--tw-shadow-color": "#0284c7",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "152")]
#[test_case(r#####"tw`shadow-sky-700`"#####, r#####"({
  "--tw-shadow-color": "#0369a1",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "153")]
#[test_case(r#####"tw`shadow-sky-800`"#####, r#####"({
  "--tw-shadow-color": "#075985",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "154")]
#[test_case(r#####"tw`shadow-sky-900`"#####, r#####"({
  "--tw-shadow-color": "#0c4a6e",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "155")]
#[test_case(r#####"tw`shadow-blue-50`"#####, r#####"({
  "--tw-shadow-color": "#eff6ff",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "156")]
#[test_case(r#####"tw`shadow-blue-100`"#####, r#####"({
  "--tw-shadow-color": "#dbeafe",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "157")]
#[test_case(r#####"tw`shadow-blue-200`"#####, r#####"({
  "--tw-shadow-color": "#bfdbfe",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "158")]
#[test_case(r#####"tw`shadow-blue-300`"#####, r#####"({
  "--tw-shadow-color": "#93c5fd",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "159")]
#[test_case(r#####"tw`shadow-blue-400`"#####, r#####"({
  "--tw-shadow-color": "#60a5fa",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "160")]
#[test_case(r#####"tw`shadow-blue-500`"#####, r#####"({
  "--tw-shadow-color": "#3b82f6",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "161")]
#[test_case(r#####"tw`shadow-blue-600`"#####, r#####"({
  "--tw-shadow-color": "#2563eb",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "162")]
#[test_case(r#####"tw`shadow-blue-700`"#####, r#####"({
  "--tw-shadow-color": "#1d4ed8",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "163")]
#[test_case(r#####"tw`shadow-blue-800`"#####, r#####"({
  "--tw-shadow-color": "#1e40af",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "164")]
#[test_case(r#####"tw`shadow-blue-900`"#####, r#####"({
  "--tw-shadow-color": "#1e3a8a",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "165")]
#[test_case(r#####"tw`shadow-indigo-50`"#####, r#####"({
  "--tw-shadow-color": "#eef2ff",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "166")]
#[test_case(r#####"tw`shadow-indigo-100`"#####, r#####"({
  "--tw-shadow-color": "#e0e7ff",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "167")]
#[test_case(r#####"tw`shadow-indigo-200`"#####, r#####"({
  "--tw-shadow-color": "#c7d2fe",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "168")]
#[test_case(r#####"tw`shadow-indigo-300`"#####, r#####"({
  "--tw-shadow-color": "#a5b4fc",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "169")]
#[test_case(r#####"tw`shadow-indigo-400`"#####, r#####"({
  "--tw-shadow-color": "#818cf8",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "170")]
#[test_case(r#####"tw`shadow-indigo-500`"#####, r#####"({
  "--tw-shadow-color": "#6366f1",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "171")]
#[test_case(r#####"tw`shadow-indigo-600`"#####, r#####"({
  "--tw-shadow-color": "#4f46e5",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "172")]
#[test_case(r#####"tw`shadow-indigo-700`"#####, r#####"({
  "--tw-shadow-color": "#4338ca",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "173")]
#[test_case(r#####"tw`shadow-indigo-800`"#####, r#####"({
  "--tw-shadow-color": "#3730a3",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "174")]
#[test_case(r#####"tw`shadow-indigo-900`"#####, r#####"({
  "--tw-shadow-color": "#312e81",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "175")]
#[test_case(r#####"tw`shadow-violet-50`"#####, r#####"({
  "--tw-shadow-color": "#f5f3ff",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "176")]
#[test_case(r#####"tw`shadow-violet-100`"#####, r#####"({
  "--tw-shadow-color": "#ede9fe",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "177")]
#[test_case(r#####"tw`shadow-violet-200`"#####, r#####"({
  "--tw-shadow-color": "#ddd6fe",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "178")]
#[test_case(r#####"tw`shadow-violet-300`"#####, r#####"({
  "--tw-shadow-color": "#c4b5fd",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "179")]
#[test_case(r#####"tw`shadow-violet-400`"#####, r#####"({
  "--tw-shadow-color": "#a78bfa",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "180")]
#[test_case(r#####"tw`shadow-violet-500`"#####, r#####"({
  "--tw-shadow-color": "#8b5cf6",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "181")]
#[test_case(r#####"tw`shadow-violet-600`"#####, r#####"({
  "--tw-shadow-color": "#7c3aed",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "182")]
#[test_case(r#####"tw`shadow-violet-700`"#####, r#####"({
  "--tw-shadow-color": "#6d28d9",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "183")]
#[test_case(r#####"tw`shadow-violet-800`"#####, r#####"({
  "--tw-shadow-color": "#5b21b6",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "184")]
#[test_case(r#####"tw`shadow-violet-900`"#####, r#####"({
  "--tw-shadow-color": "#4c1d95",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "185")]
#[test_case(r#####"tw`shadow-purple-50`"#####, r#####"({
  "--tw-shadow-color": "#faf5ff",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "186")]
#[test_case(r#####"tw`shadow-purple-100`"#####, r#####"({
  "--tw-shadow-color": "#f3e8ff",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "187")]
#[test_case(r#####"tw`shadow-purple-200`"#####, r#####"({
  "--tw-shadow-color": "#e9d5ff",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "188")]
#[test_case(r#####"tw`shadow-purple-300`"#####, r#####"({
  "--tw-shadow-color": "#d8b4fe",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "189")]
#[test_case(r#####"tw`shadow-purple-400`"#####, r#####"({
  "--tw-shadow-color": "#c084fc",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "190")]
#[test_case(r#####"tw`shadow-purple-500`"#####, r#####"({
  "--tw-shadow-color": "#a855f7",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "191")]
#[test_case(r#####"tw`shadow-purple-600`"#####, r#####"({
  "--tw-shadow-color": "#9333ea",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "192")]
#[test_case(r#####"tw`shadow-purple-700`"#####, r#####"({
  "--tw-shadow-color": "#7e22ce",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "193")]
#[test_case(r#####"tw`shadow-purple-800`"#####, r#####"({
  "--tw-shadow-color": "#6b21a8",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "194")]
#[test_case(r#####"tw`shadow-purple-900`"#####, r#####"({
  "--tw-shadow-color": "#581c87",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "195")]
#[test_case(r#####"tw`shadow-fuchsia-50`"#####, r#####"({
  "--tw-shadow-color": "#fdf4ff",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "196")]
#[test_case(r#####"tw`shadow-fuchsia-100`"#####, r#####"({
  "--tw-shadow-color": "#fae8ff",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "197")]
#[test_case(r#####"tw`shadow-fuchsia-200`"#####, r#####"({
  "--tw-shadow-color": "#f5d0fe",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "198")]
#[test_case(r#####"tw`shadow-fuchsia-300`"#####, r#####"({
  "--tw-shadow-color": "#f0abfc",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "199")]
#[test_case(r#####"tw`shadow-fuchsia-400`"#####, r#####"({
  "--tw-shadow-color": "#e879f9",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "200")]
#[test_case(r#####"tw`shadow-fuchsia-500`"#####, r#####"({
  "--tw-shadow-color": "#d946ef",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "201")]
#[test_case(r#####"tw`shadow-fuchsia-600`"#####, r#####"({
  "--tw-shadow-color": "#c026d3",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "202")]
#[test_case(r#####"tw`shadow-fuchsia-700`"#####, r#####"({
  "--tw-shadow-color": "#a21caf",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "203")]
#[test_case(r#####"tw`shadow-fuchsia-800`"#####, r#####"({
  "--tw-shadow-color": "#86198f",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "204")]
#[test_case(r#####"tw`shadow-fuchsia-900`"#####, r#####"({
  "--tw-shadow-color": "#701a75",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "205")]
#[test_case(r#####"tw`shadow-pink-50`"#####, r#####"({
  "--tw-shadow-color": "#fdf2f8",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "206")]
#[test_case(r#####"tw`shadow-pink-100`"#####, r#####"({
  "--tw-shadow-color": "#fce7f3",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "207")]
#[test_case(r#####"tw`shadow-pink-200`"#####, r#####"({
  "--tw-shadow-color": "#fbcfe8",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "208")]
#[test_case(r#####"tw`shadow-pink-300`"#####, r#####"({
  "--tw-shadow-color": "#f9a8d4",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "209")]
#[test_case(r#####"tw`shadow-pink-400`"#####, r#####"({
  "--tw-shadow-color": "#f472b6",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "210")]
#[test_case(r#####"tw`shadow-pink-500`"#####, r#####"({
  "--tw-shadow-color": "#ec4899",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "211")]
#[test_case(r#####"tw`shadow-pink-600`"#####, r#####"({
  "--tw-shadow-color": "#db2777",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "212")]
#[test_case(r#####"tw`shadow-pink-700`"#####, r#####"({
  "--tw-shadow-color": "#be185d",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "213")]
#[test_case(r#####"tw`shadow-pink-800`"#####, r#####"({
  "--tw-shadow-color": "#9d174d",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "214")]
#[test_case(r#####"tw`shadow-pink-900`"#####, r#####"({
  "--tw-shadow-color": "#831843",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "215")]
#[test_case(r#####"tw`shadow-rose-50`"#####, r#####"({
  "--tw-shadow-color": "#fff1f2",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "216")]
#[test_case(r#####"tw`shadow-rose-100`"#####, r#####"({
  "--tw-shadow-color": "#ffe4e6",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "217")]
#[test_case(r#####"tw`shadow-rose-200`"#####, r#####"({
  "--tw-shadow-color": "#fecdd3",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "218")]
#[test_case(r#####"tw`shadow-rose-300`"#####, r#####"({
  "--tw-shadow-color": "#fda4af",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "219")]
#[test_case(r#####"tw`shadow-rose-400`"#####, r#####"({
  "--tw-shadow-color": "#fb7185",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "220")]
#[test_case(r#####"tw`shadow-rose-500`"#####, r#####"({
  "--tw-shadow-color": "#f43f5e",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "221")]
#[test_case(r#####"tw`shadow-rose-600`"#####, r#####"({
  "--tw-shadow-color": "#e11d48",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "222")]
#[test_case(r#####"tw`shadow-rose-700`"#####, r#####"({
  "--tw-shadow-color": "#be123c",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "223")]
#[test_case(r#####"tw`shadow-rose-800`"#####, r#####"({
  "--tw-shadow-color": "#9f1239",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "224")]
#[test_case(r#####"tw`shadow-rose-900`"#####, r#####"({
  "--tw-shadow-color": "#881337",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "225")]
#[test_case(r#####"tw`shadow-cyan-500/50`"#####, r#####"({
  "--tw-shadow-color": "rgb(6 182 212 / 0.5)",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "226")]
#[test_case(r#####"tw`shadow-cyan-500/[.50]`"#####, r#####"({
  "--tw-shadow-color": "rgb(6 182 212 / .50)",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "227")]
#[test_case(r#####"tw`shadow-[#50d71e]`"#####, r#####"({
  "--tw-shadow-color": "#50d71e",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "228")]
#[test_case(r#####"tw`shadow-[color:#50d71e]`"#####, r#####"({
  "--tw-shadow-color": "#50d71e",
  "--tw-shadow": "var(--tw-shadow-colored)",
})
;"##### ; "229")]
#[test_case(r#####"tw`shadow-[shadow:#50d71e]`"#####, r#####"({
  "--tw-shadow": "#50d71e",
  "--tw-shadow-colored": "#50d71e",
  boxShadow:
    "var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)",
})"##### ; "230")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
