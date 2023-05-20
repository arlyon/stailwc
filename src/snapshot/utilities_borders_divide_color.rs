use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`divideColor.`"#####, r#####"({
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
;"##### ; "1")]
#[test_case(r#####"tw`divide-inherit`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    borderColor: "inherit",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`divide-current`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    borderColor: "currentColor",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`divide-transparent`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    borderColor: "transparent",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`divide-black`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(0 0 0 / var(--tw-divide-opacity))",
  },
})
;"##### ; "5")]
#[test_case(r#####"tw`divide-white`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(255 255 255 / var(--tw-divide-opacity))",
  },
})
;"##### ; "6")]
#[test_case(r#####"tw`divide-slate-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(248 250 252 / var(--tw-divide-opacity))",
  },
})
;"##### ; "7")]
#[test_case(r#####"tw`divide-slate-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(241 245 249 / var(--tw-divide-opacity))",
  },
})
;"##### ; "8")]
#[test_case(r#####"tw`divide-slate-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(226 232 240 / var(--tw-divide-opacity))",
  },
})
;"##### ; "9")]
#[test_case(r#####"tw`divide-slate-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(203 213 225 / var(--tw-divide-opacity))",
  },
})
;"##### ; "10")]
#[test_case(r#####"tw`divide-slate-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(148 163 184 / var(--tw-divide-opacity))",
  },
})
;"##### ; "11")]
#[test_case(r#####"tw`divide-slate-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(100 116 139 / var(--tw-divide-opacity))",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`divide-slate-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(71 85 105 / var(--tw-divide-opacity))",
  },
})
;"##### ; "13")]
#[test_case(r#####"tw`divide-slate-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(51 65 85 / var(--tw-divide-opacity))",
  },
})
;"##### ; "14")]
#[test_case(r#####"tw`divide-slate-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(30 41 59 / var(--tw-divide-opacity))",
  },
})
;"##### ; "15")]
#[test_case(r#####"tw`divide-slate-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(15 23 42 / var(--tw-divide-opacity))",
  },
})
;"##### ; "16")]
#[test_case(r#####"tw`divide-gray-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(249 250 251 / var(--tw-divide-opacity))",
  },
})
;"##### ; "17")]
#[test_case(r#####"tw`divide-gray-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(243 244 246 / var(--tw-divide-opacity))",
  },
})
;"##### ; "18")]
#[test_case(r#####"tw`divide-gray-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(229 231 235 / var(--tw-divide-opacity))",
  },
})
;"##### ; "19")]
#[test_case(r#####"tw`divide-gray-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(209 213 219 / var(--tw-divide-opacity))",
  },
})
;"##### ; "20")]
#[test_case(r#####"tw`divide-gray-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(156 163 175 / var(--tw-divide-opacity))",
  },
})
;"##### ; "21")]
#[test_case(r#####"tw`divide-gray-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(107 114 128 / var(--tw-divide-opacity))",
  },
})
;"##### ; "22")]
#[test_case(r#####"tw`divide-gray-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(75 85 99 / var(--tw-divide-opacity))",
  },
})
;"##### ; "23")]
#[test_case(r#####"tw`divide-gray-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(55 65 81 / var(--tw-divide-opacity))",
  },
})
;"##### ; "24")]
#[test_case(r#####"tw`divide-gray-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(31 41 55 / var(--tw-divide-opacity))",
  },
})
;"##### ; "25")]
#[test_case(r#####"tw`divide-gray-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(17 24 39 / var(--tw-divide-opacity))",
  },
})
;"##### ; "26")]
#[test_case(r#####"tw`divide-zinc-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(250 250 250 / var(--tw-divide-opacity))",
  },
})
;"##### ; "27")]
#[test_case(r#####"tw`divide-zinc-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(244 244 245 / var(--tw-divide-opacity))",
  },
})
;"##### ; "28")]
#[test_case(r#####"tw`divide-zinc-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(228 228 231 / var(--tw-divide-opacity))",
  },
})
;"##### ; "29")]
#[test_case(r#####"tw`divide-zinc-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(212 212 216 / var(--tw-divide-opacity))",
  },
})
;"##### ; "30")]
#[test_case(r#####"tw`divide-zinc-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(161 161 170 / var(--tw-divide-opacity))",
  },
})
;"##### ; "31")]
#[test_case(r#####"tw`divide-zinc-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(113 113 122 / var(--tw-divide-opacity))",
  },
})
;"##### ; "32")]
#[test_case(r#####"tw`divide-zinc-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(82 82 91 / var(--tw-divide-opacity))",
  },
})
;"##### ; "33")]
#[test_case(r#####"tw`divide-zinc-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(63 63 70 / var(--tw-divide-opacity))",
  },
})
;"##### ; "34")]
#[test_case(r#####"tw`divide-zinc-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(39 39 42 / var(--tw-divide-opacity))",
  },
})
;"##### ; "35")]
#[test_case(r#####"tw`divide-zinc-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(24 24 27 / var(--tw-divide-opacity))",
  },
})
;"##### ; "36")]
#[test_case(r#####"tw`divide-neutral-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(250 250 250 / var(--tw-divide-opacity))",
  },
})
;"##### ; "37")]
#[test_case(r#####"tw`divide-neutral-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(245 245 245 / var(--tw-divide-opacity))",
  },
})
;"##### ; "38")]
#[test_case(r#####"tw`divide-neutral-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(229 229 229 / var(--tw-divide-opacity))",
  },
})
;"##### ; "39")]
#[test_case(r#####"tw`divide-neutral-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(212 212 212 / var(--tw-divide-opacity))",
  },
})
;"##### ; "40")]
#[test_case(r#####"tw`divide-neutral-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(163 163 163 / var(--tw-divide-opacity))",
  },
})
;"##### ; "41")]
#[test_case(r#####"tw`divide-neutral-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(115 115 115 / var(--tw-divide-opacity))",
  },
})
;"##### ; "42")]
#[test_case(r#####"tw`divide-neutral-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(82 82 82 / var(--tw-divide-opacity))",
  },
})
;"##### ; "43")]
#[test_case(r#####"tw`divide-neutral-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(64 64 64 / var(--tw-divide-opacity))",
  },
})
;"##### ; "44")]
#[test_case(r#####"tw`divide-neutral-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(38 38 38 / var(--tw-divide-opacity))",
  },
})
;"##### ; "45")]
#[test_case(r#####"tw`divide-neutral-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(23 23 23 / var(--tw-divide-opacity))",
  },
})
;"##### ; "46")]
#[test_case(r#####"tw`divide-stone-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(250 250 249 / var(--tw-divide-opacity))",
  },
})
;"##### ; "47")]
#[test_case(r#####"tw`divide-stone-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(245 245 244 / var(--tw-divide-opacity))",
  },
})
;"##### ; "48")]
#[test_case(r#####"tw`divide-stone-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(231 229 228 / var(--tw-divide-opacity))",
  },
})
;"##### ; "49")]
#[test_case(r#####"tw`divide-stone-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(214 211 209 / var(--tw-divide-opacity))",
  },
})
;"##### ; "50")]
#[test_case(r#####"tw`divide-stone-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(168 162 158 / var(--tw-divide-opacity))",
  },
})
;"##### ; "51")]
#[test_case(r#####"tw`divide-stone-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(120 113 108 / var(--tw-divide-opacity))",
  },
})
;"##### ; "52")]
#[test_case(r#####"tw`divide-stone-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(87 83 78 / var(--tw-divide-opacity))",
  },
})
;"##### ; "53")]
#[test_case(r#####"tw`divide-stone-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(68 64 60 / var(--tw-divide-opacity))",
  },
})
;"##### ; "54")]
#[test_case(r#####"tw`divide-stone-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(41 37 36 / var(--tw-divide-opacity))",
  },
})
;"##### ; "55")]
#[test_case(r#####"tw`divide-stone-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(28 25 23 / var(--tw-divide-opacity))",
  },
})
;"##### ; "56")]
#[test_case(r#####"tw`divide-red-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(254 242 242 / var(--tw-divide-opacity))",
  },
})
;"##### ; "57")]
#[test_case(r#####"tw`divide-red-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(254 226 226 / var(--tw-divide-opacity))",
  },
})
;"##### ; "58")]
#[test_case(r#####"tw`divide-red-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(254 202 202 / var(--tw-divide-opacity))",
  },
})
;"##### ; "59")]
#[test_case(r#####"tw`divide-red-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(252 165 165 / var(--tw-divide-opacity))",
  },
})
;"##### ; "60")]
#[test_case(r#####"tw`divide-red-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(248 113 113 / var(--tw-divide-opacity))",
  },
})
;"##### ; "61")]
#[test_case(r#####"tw`divide-red-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(239 68 68 / var(--tw-divide-opacity))",
  },
})
;"##### ; "62")]
#[test_case(r#####"tw`divide-red-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(220 38 38 / var(--tw-divide-opacity))",
  },
})
;"##### ; "63")]
#[test_case(r#####"tw`divide-red-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(185 28 28 / var(--tw-divide-opacity))",
  },
})
;"##### ; "64")]
#[test_case(r#####"tw`divide-red-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(153 27 27 / var(--tw-divide-opacity))",
  },
})
;"##### ; "65")]
#[test_case(r#####"tw`divide-red-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(127 29 29 / var(--tw-divide-opacity))",
  },
})
;"##### ; "66")]
#[test_case(r#####"tw`divide-orange-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(255 247 237 / var(--tw-divide-opacity))",
  },
})
;"##### ; "67")]
#[test_case(r#####"tw`divide-orange-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(255 237 213 / var(--tw-divide-opacity))",
  },
})
;"##### ; "68")]
#[test_case(r#####"tw`divide-orange-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(254 215 170 / var(--tw-divide-opacity))",
  },
})
;"##### ; "69")]
#[test_case(r#####"tw`divide-orange-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(253 186 116 / var(--tw-divide-opacity))",
  },
})
;"##### ; "70")]
#[test_case(r#####"tw`divide-orange-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(251 146 60 / var(--tw-divide-opacity))",
  },
})
;"##### ; "71")]
#[test_case(r#####"tw`divide-orange-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(249 115 22 / var(--tw-divide-opacity))",
  },
})
;"##### ; "72")]
#[test_case(r#####"tw`divide-orange-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(234 88 12 / var(--tw-divide-opacity))",
  },
})
;"##### ; "73")]
#[test_case(r#####"tw`divide-orange-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(194 65 12 / var(--tw-divide-opacity))",
  },
})
;"##### ; "74")]
#[test_case(r#####"tw`divide-orange-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(154 52 18 / var(--tw-divide-opacity))",
  },
})
;"##### ; "75")]
#[test_case(r#####"tw`divide-orange-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(124 45 18 / var(--tw-divide-opacity))",
  },
})
;"##### ; "76")]
#[test_case(r#####"tw`divide-amber-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(255 251 235 / var(--tw-divide-opacity))",
  },
})
;"##### ; "77")]
#[test_case(r#####"tw`divide-amber-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(254 243 199 / var(--tw-divide-opacity))",
  },
})
;"##### ; "78")]
#[test_case(r#####"tw`divide-amber-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(253 230 138 / var(--tw-divide-opacity))",
  },
})
;"##### ; "79")]
#[test_case(r#####"tw`divide-amber-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(252 211 77 / var(--tw-divide-opacity))",
  },
})
;"##### ; "80")]
#[test_case(r#####"tw`divide-amber-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(251 191 36 / var(--tw-divide-opacity))",
  },
})
;"##### ; "81")]
#[test_case(r#####"tw`divide-amber-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(245 158 11 / var(--tw-divide-opacity))",
  },
})
;"##### ; "82")]
#[test_case(r#####"tw`divide-amber-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(217 119 6 / var(--tw-divide-opacity))",
  },
})
;"##### ; "83")]
#[test_case(r#####"tw`divide-amber-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(180 83 9 / var(--tw-divide-opacity))",
  },
})
;"##### ; "84")]
#[test_case(r#####"tw`divide-amber-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(146 64 14 / var(--tw-divide-opacity))",
  },
})
;"##### ; "85")]
#[test_case(r#####"tw`divide-amber-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(120 53 15 / var(--tw-divide-opacity))",
  },
})
;"##### ; "86")]
#[test_case(r#####"tw`divide-yellow-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(254 252 232 / var(--tw-divide-opacity))",
  },
})
;"##### ; "87")]
#[test_case(r#####"tw`divide-yellow-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(254 249 195 / var(--tw-divide-opacity))",
  },
})
;"##### ; "88")]
#[test_case(r#####"tw`divide-yellow-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(254 240 138 / var(--tw-divide-opacity))",
  },
})
;"##### ; "89")]
#[test_case(r#####"tw`divide-yellow-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(253 224 71 / var(--tw-divide-opacity))",
  },
})
;"##### ; "90")]
#[test_case(r#####"tw`divide-yellow-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(250 204 21 / var(--tw-divide-opacity))",
  },
})
;"##### ; "91")]
#[test_case(r#####"tw`divide-yellow-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(234 179 8 / var(--tw-divide-opacity))",
  },
})
;"##### ; "92")]
#[test_case(r#####"tw`divide-yellow-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(202 138 4 / var(--tw-divide-opacity))",
  },
})
;"##### ; "93")]
#[test_case(r#####"tw`divide-yellow-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(161 98 7 / var(--tw-divide-opacity))",
  },
})
;"##### ; "94")]
#[test_case(r#####"tw`divide-yellow-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(133 77 14 / var(--tw-divide-opacity))",
  },
})
;"##### ; "95")]
#[test_case(r#####"tw`divide-yellow-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(113 63 18 / var(--tw-divide-opacity))",
  },
})
;"##### ; "96")]
#[test_case(r#####"tw`divide-lime-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(247 254 231 / var(--tw-divide-opacity))",
  },
})
;"##### ; "97")]
#[test_case(r#####"tw`divide-lime-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(236 252 203 / var(--tw-divide-opacity))",
  },
})
;"##### ; "98")]
#[test_case(r#####"tw`divide-lime-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(217 249 157 / var(--tw-divide-opacity))",
  },
})
;"##### ; "99")]
#[test_case(r#####"tw`divide-lime-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(190 242 100 / var(--tw-divide-opacity))",
  },
})
;"##### ; "100")]
#[test_case(r#####"tw`divide-lime-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(163 230 53 / var(--tw-divide-opacity))",
  },
})
;"##### ; "101")]
#[test_case(r#####"tw`divide-lime-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(132 204 22 / var(--tw-divide-opacity))",
  },
})
;"##### ; "102")]
#[test_case(r#####"tw`divide-lime-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(101 163 13 / var(--tw-divide-opacity))",
  },
})
;"##### ; "103")]
#[test_case(r#####"tw`divide-lime-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(77 124 15 / var(--tw-divide-opacity))",
  },
})
;"##### ; "104")]
#[test_case(r#####"tw`divide-lime-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(63 98 18 / var(--tw-divide-opacity))",
  },
})
;"##### ; "105")]
#[test_case(r#####"tw`divide-lime-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(54 83 20 / var(--tw-divide-opacity))",
  },
})
;"##### ; "106")]
#[test_case(r#####"tw`divide-green-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(240 253 244 / var(--tw-divide-opacity))",
  },
})
;"##### ; "107")]
#[test_case(r#####"tw`divide-green-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(220 252 231 / var(--tw-divide-opacity))",
  },
})
;"##### ; "108")]
#[test_case(r#####"tw`divide-green-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(187 247 208 / var(--tw-divide-opacity))",
  },
})
;"##### ; "109")]
#[test_case(r#####"tw`divide-green-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(134 239 172 / var(--tw-divide-opacity))",
  },
})
;"##### ; "110")]
#[test_case(r#####"tw`divide-green-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(74 222 128 / var(--tw-divide-opacity))",
  },
})
;"##### ; "111")]
#[test_case(r#####"tw`divide-green-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(34 197 94 / var(--tw-divide-opacity))",
  },
})
;"##### ; "112")]
#[test_case(r#####"tw`divide-green-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(22 163 74 / var(--tw-divide-opacity))",
  },
})
;"##### ; "113")]
#[test_case(r#####"tw`divide-green-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(21 128 61 / var(--tw-divide-opacity))",
  },
})
;"##### ; "114")]
#[test_case(r#####"tw`divide-green-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(22 101 52 / var(--tw-divide-opacity))",
  },
})
;"##### ; "115")]
#[test_case(r#####"tw`divide-green-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(20 83 45 / var(--tw-divide-opacity))",
  },
})
;"##### ; "116")]
#[test_case(r#####"tw`divide-emerald-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(236 253 245 / var(--tw-divide-opacity))",
  },
})
;"##### ; "117")]
#[test_case(r#####"tw`divide-emerald-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(209 250 229 / var(--tw-divide-opacity))",
  },
})
;"##### ; "118")]
#[test_case(r#####"tw`divide-emerald-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(167 243 208 / var(--tw-divide-opacity))",
  },
})
;"##### ; "119")]
#[test_case(r#####"tw`divide-emerald-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(110 231 183 / var(--tw-divide-opacity))",
  },
})
;"##### ; "120")]
#[test_case(r#####"tw`divide-emerald-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(52 211 153 / var(--tw-divide-opacity))",
  },
})
;"##### ; "121")]
#[test_case(r#####"tw`divide-emerald-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(16 185 129 / var(--tw-divide-opacity))",
  },
})
;"##### ; "122")]
#[test_case(r#####"tw`divide-emerald-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(5 150 105 / var(--tw-divide-opacity))",
  },
})
;"##### ; "123")]
#[test_case(r#####"tw`divide-emerald-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(4 120 87 / var(--tw-divide-opacity))",
  },
})
;"##### ; "124")]
#[test_case(r#####"tw`divide-emerald-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(6 95 70 / var(--tw-divide-opacity))",
  },
})
;"##### ; "125")]
#[test_case(r#####"tw`divide-emerald-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(6 78 59 / var(--tw-divide-opacity))",
  },
})
;"##### ; "126")]
#[test_case(r#####"tw`divide-teal-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(240 253 250 / var(--tw-divide-opacity))",
  },
})
;"##### ; "127")]
#[test_case(r#####"tw`divide-teal-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(204 251 241 / var(--tw-divide-opacity))",
  },
})
;"##### ; "128")]
#[test_case(r#####"tw`divide-teal-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(153 246 228 / var(--tw-divide-opacity))",
  },
})
;"##### ; "129")]
#[test_case(r#####"tw`divide-teal-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(94 234 212 / var(--tw-divide-opacity))",
  },
})
;"##### ; "130")]
#[test_case(r#####"tw`divide-teal-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(45 212 191 / var(--tw-divide-opacity))",
  },
})
;"##### ; "131")]
#[test_case(r#####"tw`divide-teal-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(20 184 166 / var(--tw-divide-opacity))",
  },
})
;"##### ; "132")]
#[test_case(r#####"tw`divide-teal-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(13 148 136 / var(--tw-divide-opacity))",
  },
})
;"##### ; "133")]
#[test_case(r#####"tw`divide-teal-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(15 118 110 / var(--tw-divide-opacity))",
  },
})
;"##### ; "134")]
#[test_case(r#####"tw`divide-teal-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(17 94 89 / var(--tw-divide-opacity))",
  },
})
;"##### ; "135")]
#[test_case(r#####"tw`divide-teal-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(19 78 74 / var(--tw-divide-opacity))",
  },
})
;"##### ; "136")]
#[test_case(r#####"tw`divide-cyan-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(236 254 255 / var(--tw-divide-opacity))",
  },
})
;"##### ; "137")]
#[test_case(r#####"tw`divide-cyan-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(207 250 254 / var(--tw-divide-opacity))",
  },
})
;"##### ; "138")]
#[test_case(r#####"tw`divide-cyan-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(165 243 252 / var(--tw-divide-opacity))",
  },
})
;"##### ; "139")]
#[test_case(r#####"tw`divide-cyan-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(103 232 249 / var(--tw-divide-opacity))",
  },
})
;"##### ; "140")]
#[test_case(r#####"tw`divide-cyan-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(34 211 238 / var(--tw-divide-opacity))",
  },
})
;"##### ; "141")]
#[test_case(r#####"tw`divide-cyan-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(6 182 212 / var(--tw-divide-opacity))",
  },
})
;"##### ; "142")]
#[test_case(r#####"tw`divide-cyan-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(8 145 178 / var(--tw-divide-opacity))",
  },
})
;"##### ; "143")]
#[test_case(r#####"tw`divide-cyan-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(14 116 144 / var(--tw-divide-opacity))",
  },
})
;"##### ; "144")]
#[test_case(r#####"tw`divide-cyan-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(21 94 117 / var(--tw-divide-opacity))",
  },
})
;"##### ; "145")]
#[test_case(r#####"tw`divide-cyan-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(22 78 99 / var(--tw-divide-opacity))",
  },
})
;"##### ; "146")]
#[test_case(r#####"tw`divide-sky-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(240 249 255 / var(--tw-divide-opacity))",
  },
})
;"##### ; "147")]
#[test_case(r#####"tw`divide-sky-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(224 242 254 / var(--tw-divide-opacity))",
  },
})
;"##### ; "148")]
#[test_case(r#####"tw`divide-sky-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(186 230 253 / var(--tw-divide-opacity))",
  },
})
;"##### ; "149")]
#[test_case(r#####"tw`divide-sky-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(125 211 252 / var(--tw-divide-opacity))",
  },
})
;"##### ; "150")]
#[test_case(r#####"tw`divide-sky-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(56 189 248 / var(--tw-divide-opacity))",
  },
})
;"##### ; "151")]
#[test_case(r#####"tw`divide-sky-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(14 165 233 / var(--tw-divide-opacity))",
  },
})
;"##### ; "152")]
#[test_case(r#####"tw`divide-sky-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(2 132 199 / var(--tw-divide-opacity))",
  },
})
;"##### ; "153")]
#[test_case(r#####"tw`divide-sky-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(3 105 161 / var(--tw-divide-opacity))",
  },
})
;"##### ; "154")]
#[test_case(r#####"tw`divide-sky-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(7 89 133 / var(--tw-divide-opacity))",
  },
})
;"##### ; "155")]
#[test_case(r#####"tw`divide-sky-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(12 74 110 / var(--tw-divide-opacity))",
  },
})
;"##### ; "156")]
#[test_case(r#####"tw`divide-blue-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(239 246 255 / var(--tw-divide-opacity))",
  },
})
;"##### ; "157")]
#[test_case(r#####"tw`divide-blue-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(219 234 254 / var(--tw-divide-opacity))",
  },
})
;"##### ; "158")]
#[test_case(r#####"tw`divide-blue-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(191 219 254 / var(--tw-divide-opacity))",
  },
})
;"##### ; "159")]
#[test_case(r#####"tw`divide-blue-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(147 197 253 / var(--tw-divide-opacity))",
  },
})
;"##### ; "160")]
#[test_case(r#####"tw`divide-blue-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(96 165 250 / var(--tw-divide-opacity))",
  },
})
;"##### ; "161")]
#[test_case(r#####"tw`divide-blue-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(59 130 246 / var(--tw-divide-opacity))",
  },
})
;"##### ; "162")]
#[test_case(r#####"tw`divide-blue-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(37 99 235 / var(--tw-divide-opacity))",
  },
})
;"##### ; "163")]
#[test_case(r#####"tw`divide-blue-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(29 78 216 / var(--tw-divide-opacity))",
  },
})
;"##### ; "164")]
#[test_case(r#####"tw`divide-blue-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(30 64 175 / var(--tw-divide-opacity))",
  },
})
;"##### ; "165")]
#[test_case(r#####"tw`divide-blue-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(30 58 138 / var(--tw-divide-opacity))",
  },
})
;"##### ; "166")]
#[test_case(r#####"tw`divide-indigo-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(238 242 255 / var(--tw-divide-opacity))",
  },
})
;"##### ; "167")]
#[test_case(r#####"tw`divide-indigo-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(224 231 255 / var(--tw-divide-opacity))",
  },
})
;"##### ; "168")]
#[test_case(r#####"tw`divide-indigo-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(199 210 254 / var(--tw-divide-opacity))",
  },
})
;"##### ; "169")]
#[test_case(r#####"tw`divide-indigo-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(165 180 252 / var(--tw-divide-opacity))",
  },
})
;"##### ; "170")]
#[test_case(r#####"tw`divide-indigo-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(129 140 248 / var(--tw-divide-opacity))",
  },
})
;"##### ; "171")]
#[test_case(r#####"tw`divide-indigo-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(99 102 241 / var(--tw-divide-opacity))",
  },
})
;"##### ; "172")]
#[test_case(r#####"tw`divide-indigo-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(79 70 229 / var(--tw-divide-opacity))",
  },
})
;"##### ; "173")]
#[test_case(r#####"tw`divide-indigo-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(67 56 202 / var(--tw-divide-opacity))",
  },
})
;"##### ; "174")]
#[test_case(r#####"tw`divide-indigo-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(55 48 163 / var(--tw-divide-opacity))",
  },
})
;"##### ; "175")]
#[test_case(r#####"tw`divide-indigo-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(49 46 129 / var(--tw-divide-opacity))",
  },
})
;"##### ; "176")]
#[test_case(r#####"tw`divide-violet-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(245 243 255 / var(--tw-divide-opacity))",
  },
})
;"##### ; "177")]
#[test_case(r#####"tw`divide-violet-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(237 233 254 / var(--tw-divide-opacity))",
  },
})
;"##### ; "178")]
#[test_case(r#####"tw`divide-violet-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(221 214 254 / var(--tw-divide-opacity))",
  },
})
;"##### ; "179")]
#[test_case(r#####"tw`divide-violet-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(196 181 253 / var(--tw-divide-opacity))",
  },
})
;"##### ; "180")]
#[test_case(r#####"tw`divide-violet-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(167 139 250 / var(--tw-divide-opacity))",
  },
})
;"##### ; "181")]
#[test_case(r#####"tw`divide-violet-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(139 92 246 / var(--tw-divide-opacity))",
  },
})
;"##### ; "182")]
#[test_case(r#####"tw`divide-violet-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(124 58 237 / var(--tw-divide-opacity))",
  },
})
;"##### ; "183")]
#[test_case(r#####"tw`divide-violet-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(109 40 217 / var(--tw-divide-opacity))",
  },
})
;"##### ; "184")]
#[test_case(r#####"tw`divide-violet-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(91 33 182 / var(--tw-divide-opacity))",
  },
})
;"##### ; "185")]
#[test_case(r#####"tw`divide-violet-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(76 29 149 / var(--tw-divide-opacity))",
  },
})
;"##### ; "186")]
#[test_case(r#####"tw`divide-purple-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(250 245 255 / var(--tw-divide-opacity))",
  },
})
;"##### ; "187")]
#[test_case(r#####"tw`divide-purple-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(243 232 255 / var(--tw-divide-opacity))",
  },
})
;"##### ; "188")]
#[test_case(r#####"tw`divide-purple-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(233 213 255 / var(--tw-divide-opacity))",
  },
})
;"##### ; "189")]
#[test_case(r#####"tw`divide-purple-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(216 180 254 / var(--tw-divide-opacity))",
  },
})
;"##### ; "190")]
#[test_case(r#####"tw`divide-purple-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(192 132 252 / var(--tw-divide-opacity))",
  },
})
;"##### ; "191")]
#[test_case(r#####"tw`divide-purple-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(168 85 247 / var(--tw-divide-opacity))",
  },
})
;"##### ; "192")]
#[test_case(r#####"tw`divide-purple-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(147 51 234 / var(--tw-divide-opacity))",
  },
})
;"##### ; "193")]
#[test_case(r#####"tw`divide-purple-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(126 34 206 / var(--tw-divide-opacity))",
  },
})
;"##### ; "194")]
#[test_case(r#####"tw`divide-purple-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(107 33 168 / var(--tw-divide-opacity))",
  },
})
;"##### ; "195")]
#[test_case(r#####"tw`divide-purple-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(88 28 135 / var(--tw-divide-opacity))",
  },
})
;"##### ; "196")]
#[test_case(r#####"tw`divide-fuchsia-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(253 244 255 / var(--tw-divide-opacity))",
  },
})
;"##### ; "197")]
#[test_case(r#####"tw`divide-fuchsia-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(250 232 255 / var(--tw-divide-opacity))",
  },
})
;"##### ; "198")]
#[test_case(r#####"tw`divide-fuchsia-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(245 208 254 / var(--tw-divide-opacity))",
  },
})
;"##### ; "199")]
#[test_case(r#####"tw`divide-fuchsia-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(240 171 252 / var(--tw-divide-opacity))",
  },
})
;"##### ; "200")]
#[test_case(r#####"tw`divide-fuchsia-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(232 121 249 / var(--tw-divide-opacity))",
  },
})
;"##### ; "201")]
#[test_case(r#####"tw`divide-fuchsia-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(217 70 239 / var(--tw-divide-opacity))",
  },
})
;"##### ; "202")]
#[test_case(r#####"tw`divide-fuchsia-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(192 38 211 / var(--tw-divide-opacity))",
  },
})
;"##### ; "203")]
#[test_case(r#####"tw`divide-fuchsia-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(162 28 175 / var(--tw-divide-opacity))",
  },
})
;"##### ; "204")]
#[test_case(r#####"tw`divide-fuchsia-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(134 25 143 / var(--tw-divide-opacity))",
  },
})
;"##### ; "205")]
#[test_case(r#####"tw`divide-fuchsia-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(112 26 117 / var(--tw-divide-opacity))",
  },
})
;"##### ; "206")]
#[test_case(r#####"tw`divide-pink-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(253 242 248 / var(--tw-divide-opacity))",
  },
})
;"##### ; "207")]
#[test_case(r#####"tw`divide-pink-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(252 231 243 / var(--tw-divide-opacity))",
  },
})
;"##### ; "208")]
#[test_case(r#####"tw`divide-pink-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(251 207 232 / var(--tw-divide-opacity))",
  },
})
;"##### ; "209")]
#[test_case(r#####"tw`divide-pink-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(249 168 212 / var(--tw-divide-opacity))",
  },
})
;"##### ; "210")]
#[test_case(r#####"tw`divide-pink-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(244 114 182 / var(--tw-divide-opacity))",
  },
})
;"##### ; "211")]
#[test_case(r#####"tw`divide-pink-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(236 72 153 / var(--tw-divide-opacity))",
  },
})
;"##### ; "212")]
#[test_case(r#####"tw`divide-pink-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(219 39 119 / var(--tw-divide-opacity))",
  },
})
;"##### ; "213")]
#[test_case(r#####"tw`divide-pink-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(190 24 93 / var(--tw-divide-opacity))",
  },
})
;"##### ; "214")]
#[test_case(r#####"tw`divide-pink-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(157 23 77 / var(--tw-divide-opacity))",
  },
})
;"##### ; "215")]
#[test_case(r#####"tw`divide-pink-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(131 24 67 / var(--tw-divide-opacity))",
  },
})
;"##### ; "216")]
#[test_case(r#####"tw`divide-rose-50`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(255 241 242 / var(--tw-divide-opacity))",
  },
})
;"##### ; "217")]
#[test_case(r#####"tw`divide-rose-100`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(255 228 230 / var(--tw-divide-opacity))",
  },
})
;"##### ; "218")]
#[test_case(r#####"tw`divide-rose-200`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(254 205 211 / var(--tw-divide-opacity))",
  },
})
;"##### ; "219")]
#[test_case(r#####"tw`divide-rose-300`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(253 164 175 / var(--tw-divide-opacity))",
  },
})
;"##### ; "220")]
#[test_case(r#####"tw`divide-rose-400`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(251 113 133 / var(--tw-divide-opacity))",
  },
})
;"##### ; "221")]
#[test_case(r#####"tw`divide-rose-500`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(244 63 94 / var(--tw-divide-opacity))",
  },
})
;"##### ; "222")]
#[test_case(r#####"tw`divide-rose-600`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(225 29 72 / var(--tw-divide-opacity))",
  },
})
;"##### ; "223")]
#[test_case(r#####"tw`divide-rose-700`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(190 18 60 / var(--tw-divide-opacity))",
  },
})
;"##### ; "224")]
#[test_case(r#####"tw`divide-rose-800`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(159 18 57 / var(--tw-divide-opacity))",
  },
})
;"##### ; "225")]
#[test_case(r#####"tw`divide-rose-900`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(136 19 55 / var(--tw-divide-opacity))",
  },
})
;"##### ; "226")]
#[test_case(r#####"tw`divide-y-4 divide-slate-400/25`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': "0",
    borderTopWidth: "calc(4px * calc(1 - var(--tw-divide-y-reverse)))",
    borderBottomWidth: "calc(4px * var(--tw-divide-y-reverse))",
    borderColor: "rgb(148 163 184 / 0.25)",
  },
})
;"##### ; "227")]
#[test_case(r#####"tw`divide-y-4 divide-slate-400/[.24]`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': "0",
    borderTopWidth: "calc(4px * calc(1 - var(--tw-divide-y-reverse)))",
    borderBottomWidth: "calc(4px * var(--tw-divide-y-reverse))",
    borderColor: "rgb(148 163 184 / .24)",
  },
})
;"##### ; "228")]
#[test_case(r#####"tw`divide-[#243c5a]`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(36 60 90 / var(--tw-divide-opacity))",
  },
})
;"##### ; "229")]
#[test_case(r#####"tw`divide-[color:blue]`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-opacity': "1",
    borderColor: "rgb(0 0 255 / var(--tw-divide-opacity))",
  },
})"##### ; "230")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
