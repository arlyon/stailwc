use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`placeholder-inherit`"#####, r#####"({
  '::placeholder': {
    color: "inherit",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`placeholder-current`"#####, r#####"({
  '::placeholder': {
    color: "currentColor",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`placeholder-transparent`"#####, r#####"({
  '::placeholder': {
    color: "transparent",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`placeholder-black`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(0 0 0 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`placeholder-white`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(255 255 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`placeholder-slate-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(248 250 252 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "5")]
#[test_case(r#####"tw`placeholder-slate-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(241 245 249 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "6")]
#[test_case(r#####"tw`placeholder-slate-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(226 232 240 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "7")]
#[test_case(r#####"tw`placeholder-slate-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(203 213 225 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "8")]
#[test_case(r#####"tw`placeholder-slate-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(148 163 184 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "9")]
#[test_case(r#####"tw`placeholder-slate-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(100 116 139 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "10")]
#[test_case(r#####"tw`placeholder-slate-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(71 85 105 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "11")]
#[test_case(r#####"tw`placeholder-slate-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(51 65 85 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`placeholder-slate-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(30 41 59 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "13")]
#[test_case(r#####"tw`placeholder-slate-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(15 23 42 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "14")]
#[test_case(r#####"tw`placeholder-gray-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(249 250 251 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "15")]
#[test_case(r#####"tw`placeholder-gray-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(243 244 246 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "16")]
#[test_case(r#####"tw`placeholder-gray-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(229 231 235 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "17")]
#[test_case(r#####"tw`placeholder-gray-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(209 213 219 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "18")]
#[test_case(r#####"tw`placeholder-gray-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(156 163 175 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "19")]
#[test_case(r#####"tw`placeholder-gray-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(107 114 128 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "20")]
#[test_case(r#####"tw`placeholder-gray-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(75 85 99 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "21")]
#[test_case(r#####"tw`placeholder-gray-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(55 65 81 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "22")]
#[test_case(r#####"tw`placeholder-gray-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(31 41 55 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "23")]
#[test_case(r#####"tw`placeholder-gray-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(17 24 39 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "24")]
#[test_case(r#####"tw`placeholder-zinc-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(250 250 250 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "25")]
#[test_case(r#####"tw`placeholder-zinc-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(244 244 245 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "26")]
#[test_case(r#####"tw`placeholder-zinc-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(228 228 231 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "27")]
#[test_case(r#####"tw`placeholder-zinc-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(212 212 216 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "28")]
#[test_case(r#####"tw`placeholder-zinc-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(161 161 170 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "29")]
#[test_case(r#####"tw`placeholder-zinc-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(113 113 122 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "30")]
#[test_case(r#####"tw`placeholder-zinc-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(82 82 91 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "31")]
#[test_case(r#####"tw`placeholder-zinc-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(63 63 70 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "32")]
#[test_case(r#####"tw`placeholder-zinc-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(39 39 42 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "33")]
#[test_case(r#####"tw`placeholder-zinc-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(24 24 27 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "34")]
#[test_case(r#####"tw`placeholder-neutral-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(250 250 250 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "35")]
#[test_case(r#####"tw`placeholder-neutral-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(245 245 245 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "36")]
#[test_case(r#####"tw`placeholder-neutral-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(229 229 229 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "37")]
#[test_case(r#####"tw`placeholder-neutral-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(212 212 212 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "38")]
#[test_case(r#####"tw`placeholder-neutral-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(163 163 163 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "39")]
#[test_case(r#####"tw`placeholder-neutral-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(115 115 115 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "40")]
#[test_case(r#####"tw`placeholder-neutral-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(82 82 82 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "41")]
#[test_case(r#####"tw`placeholder-neutral-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(64 64 64 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "42")]
#[test_case(r#####"tw`placeholder-neutral-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(38 38 38 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "43")]
#[test_case(r#####"tw`placeholder-neutral-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(23 23 23 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "44")]
#[test_case(r#####"tw`placeholder-stone-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(250 250 249 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "45")]
#[test_case(r#####"tw`placeholder-stone-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(245 245 244 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "46")]
#[test_case(r#####"tw`placeholder-stone-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(231 229 228 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "47")]
#[test_case(r#####"tw`placeholder-stone-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(214 211 209 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "48")]
#[test_case(r#####"tw`placeholder-stone-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(168 162 158 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "49")]
#[test_case(r#####"tw`placeholder-stone-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(120 113 108 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "50")]
#[test_case(r#####"tw`placeholder-stone-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(87 83 78 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "51")]
#[test_case(r#####"tw`placeholder-stone-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(68 64 60 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "52")]
#[test_case(r#####"tw`placeholder-stone-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(41 37 36 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "53")]
#[test_case(r#####"tw`placeholder-stone-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(28 25 23 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "54")]
#[test_case(r#####"tw`placeholder-red-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(254 242 242 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "55")]
#[test_case(r#####"tw`placeholder-red-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(254 226 226 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "56")]
#[test_case(r#####"tw`placeholder-red-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(254 202 202 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "57")]
#[test_case(r#####"tw`placeholder-red-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(252 165 165 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "58")]
#[test_case(r#####"tw`placeholder-red-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(248 113 113 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "59")]
#[test_case(r#####"tw`placeholder-red-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(239 68 68 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "60")]
#[test_case(r#####"tw`placeholder-red-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(220 38 38 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "61")]
#[test_case(r#####"tw`placeholder-red-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(185 28 28 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "62")]
#[test_case(r#####"tw`placeholder-red-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(153 27 27 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "63")]
#[test_case(r#####"tw`placeholder-red-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(127 29 29 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "64")]
#[test_case(r#####"tw`placeholder-orange-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(255 247 237 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "65")]
#[test_case(r#####"tw`placeholder-orange-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(255 237 213 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "66")]
#[test_case(r#####"tw`placeholder-orange-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(254 215 170 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "67")]
#[test_case(r#####"tw`placeholder-orange-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(253 186 116 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "68")]
#[test_case(r#####"tw`placeholder-orange-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(251 146 60 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "69")]
#[test_case(r#####"tw`placeholder-orange-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(249 115 22 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "70")]
#[test_case(r#####"tw`placeholder-orange-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(234 88 12 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "71")]
#[test_case(r#####"tw`placeholder-orange-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(194 65 12 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "72")]
#[test_case(r#####"tw`placeholder-orange-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(154 52 18 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "73")]
#[test_case(r#####"tw`placeholder-orange-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(124 45 18 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "74")]
#[test_case(r#####"tw`placeholder-amber-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(255 251 235 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "75")]
#[test_case(r#####"tw`placeholder-amber-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(254 243 199 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "76")]
#[test_case(r#####"tw`placeholder-amber-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(253 230 138 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "77")]
#[test_case(r#####"tw`placeholder-amber-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(252 211 77 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "78")]
#[test_case(r#####"tw`placeholder-amber-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(251 191 36 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "79")]
#[test_case(r#####"tw`placeholder-amber-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(245 158 11 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "80")]
#[test_case(r#####"tw`placeholder-amber-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(217 119 6 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "81")]
#[test_case(r#####"tw`placeholder-amber-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(180 83 9 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "82")]
#[test_case(r#####"tw`placeholder-amber-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(146 64 14 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "83")]
#[test_case(r#####"tw`placeholder-amber-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(120 53 15 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "84")]
#[test_case(r#####"tw`placeholder-yellow-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(254 252 232 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "85")]
#[test_case(r#####"tw`placeholder-yellow-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(254 249 195 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "86")]
#[test_case(r#####"tw`placeholder-yellow-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(254 240 138 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "87")]
#[test_case(r#####"tw`placeholder-yellow-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(253 224 71 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "88")]
#[test_case(r#####"tw`placeholder-yellow-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(250 204 21 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "89")]
#[test_case(r#####"tw`placeholder-yellow-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(234 179 8 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "90")]
#[test_case(r#####"tw`placeholder-yellow-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(202 138 4 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "91")]
#[test_case(r#####"tw`placeholder-yellow-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(161 98 7 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "92")]
#[test_case(r#####"tw`placeholder-yellow-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(133 77 14 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "93")]
#[test_case(r#####"tw`placeholder-yellow-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(113 63 18 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "94")]
#[test_case(r#####"tw`placeholder-lime-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(247 254 231 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "95")]
#[test_case(r#####"tw`placeholder-lime-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(236 252 203 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "96")]
#[test_case(r#####"tw`placeholder-lime-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(217 249 157 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "97")]
#[test_case(r#####"tw`placeholder-lime-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(190 242 100 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "98")]
#[test_case(r#####"tw`placeholder-lime-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(163 230 53 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "99")]
#[test_case(r#####"tw`placeholder-lime-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(132 204 22 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "100")]
#[test_case(r#####"tw`placeholder-lime-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(101 163 13 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "101")]
#[test_case(r#####"tw`placeholder-lime-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(77 124 15 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "102")]
#[test_case(r#####"tw`placeholder-lime-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(63 98 18 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "103")]
#[test_case(r#####"tw`placeholder-lime-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(54 83 20 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "104")]
#[test_case(r#####"tw`placeholder-green-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(240 253 244 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "105")]
#[test_case(r#####"tw`placeholder-green-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(220 252 231 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "106")]
#[test_case(r#####"tw`placeholder-green-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(187 247 208 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "107")]
#[test_case(r#####"tw`placeholder-green-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(134 239 172 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "108")]
#[test_case(r#####"tw`placeholder-green-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(74 222 128 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "109")]
#[test_case(r#####"tw`placeholder-green-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(34 197 94 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "110")]
#[test_case(r#####"tw`placeholder-green-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(22 163 74 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "111")]
#[test_case(r#####"tw`placeholder-green-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(21 128 61 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "112")]
#[test_case(r#####"tw`placeholder-green-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(22 101 52 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "113")]
#[test_case(r#####"tw`placeholder-green-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(20 83 45 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "114")]
#[test_case(r#####"tw`placeholder-emerald-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(236 253 245 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "115")]
#[test_case(r#####"tw`placeholder-emerald-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(209 250 229 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "116")]
#[test_case(r#####"tw`placeholder-emerald-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(167 243 208 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "117")]
#[test_case(r#####"tw`placeholder-emerald-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(110 231 183 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "118")]
#[test_case(r#####"tw`placeholder-emerald-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(52 211 153 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "119")]
#[test_case(r#####"tw`placeholder-emerald-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(16 185 129 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "120")]
#[test_case(r#####"tw`placeholder-emerald-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(5 150 105 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "121")]
#[test_case(r#####"tw`placeholder-emerald-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(4 120 87 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "122")]
#[test_case(r#####"tw`placeholder-emerald-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(6 95 70 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "123")]
#[test_case(r#####"tw`placeholder-emerald-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(6 78 59 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "124")]
#[test_case(r#####"tw`placeholder-teal-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(240 253 250 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "125")]
#[test_case(r#####"tw`placeholder-teal-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(204 251 241 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "126")]
#[test_case(r#####"tw`placeholder-teal-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(153 246 228 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "127")]
#[test_case(r#####"tw`placeholder-teal-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(94 234 212 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "128")]
#[test_case(r#####"tw`placeholder-teal-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(45 212 191 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "129")]
#[test_case(r#####"tw`placeholder-teal-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(20 184 166 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "130")]
#[test_case(r#####"tw`placeholder-teal-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(13 148 136 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "131")]
#[test_case(r#####"tw`placeholder-teal-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(15 118 110 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "132")]
#[test_case(r#####"tw`placeholder-teal-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(17 94 89 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "133")]
#[test_case(r#####"tw`placeholder-teal-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(19 78 74 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "134")]
#[test_case(r#####"tw`placeholder-cyan-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(236 254 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "135")]
#[test_case(r#####"tw`placeholder-cyan-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(207 250 254 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "136")]
#[test_case(r#####"tw`placeholder-cyan-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(165 243 252 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "137")]
#[test_case(r#####"tw`placeholder-cyan-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(103 232 249 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "138")]
#[test_case(r#####"tw`placeholder-cyan-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(34 211 238 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "139")]
#[test_case(r#####"tw`placeholder-cyan-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(6 182 212 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "140")]
#[test_case(r#####"tw`placeholder-cyan-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(8 145 178 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "141")]
#[test_case(r#####"tw`placeholder-cyan-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(14 116 144 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "142")]
#[test_case(r#####"tw`placeholder-cyan-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(21 94 117 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "143")]
#[test_case(r#####"tw`placeholder-cyan-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(22 78 99 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "144")]
#[test_case(r#####"tw`placeholder-sky-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(240 249 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "145")]
#[test_case(r#####"tw`placeholder-sky-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(224 242 254 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "146")]
#[test_case(r#####"tw`placeholder-sky-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(186 230 253 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "147")]
#[test_case(r#####"tw`placeholder-sky-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(125 211 252 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "148")]
#[test_case(r#####"tw`placeholder-sky-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(56 189 248 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "149")]
#[test_case(r#####"tw`placeholder-sky-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(14 165 233 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "150")]
#[test_case(r#####"tw`placeholder-sky-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(2 132 199 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "151")]
#[test_case(r#####"tw`placeholder-sky-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(3 105 161 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "152")]
#[test_case(r#####"tw`placeholder-sky-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(7 89 133 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "153")]
#[test_case(r#####"tw`placeholder-sky-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(12 74 110 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "154")]
#[test_case(r#####"tw`placeholder-blue-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(239 246 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "155")]
#[test_case(r#####"tw`placeholder-blue-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(219 234 254 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "156")]
#[test_case(r#####"tw`placeholder-blue-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(191 219 254 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "157")]
#[test_case(r#####"tw`placeholder-blue-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(147 197 253 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "158")]
#[test_case(r#####"tw`placeholder-blue-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(96 165 250 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "159")]
#[test_case(r#####"tw`placeholder-blue-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(59 130 246 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "160")]
#[test_case(r#####"tw`placeholder-blue-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(37 99 235 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "161")]
#[test_case(r#####"tw`placeholder-blue-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(29 78 216 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "162")]
#[test_case(r#####"tw`placeholder-blue-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(30 64 175 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "163")]
#[test_case(r#####"tw`placeholder-blue-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(30 58 138 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "164")]
#[test_case(r#####"tw`placeholder-indigo-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(238 242 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "165")]
#[test_case(r#####"tw`placeholder-indigo-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(224 231 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "166")]
#[test_case(r#####"tw`placeholder-indigo-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(199 210 254 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "167")]
#[test_case(r#####"tw`placeholder-indigo-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(165 180 252 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "168")]
#[test_case(r#####"tw`placeholder-indigo-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(129 140 248 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "169")]
#[test_case(r#####"tw`placeholder-indigo-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(99 102 241 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "170")]
#[test_case(r#####"tw`placeholder-indigo-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(79 70 229 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "171")]
#[test_case(r#####"tw`placeholder-indigo-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(67 56 202 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "172")]
#[test_case(r#####"tw`placeholder-indigo-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(55 48 163 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "173")]
#[test_case(r#####"tw`placeholder-indigo-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(49 46 129 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "174")]
#[test_case(r#####"tw`placeholder-violet-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(245 243 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "175")]
#[test_case(r#####"tw`placeholder-violet-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(237 233 254 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "176")]
#[test_case(r#####"tw`placeholder-violet-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(221 214 254 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "177")]
#[test_case(r#####"tw`placeholder-violet-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(196 181 253 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "178")]
#[test_case(r#####"tw`placeholder-violet-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(167 139 250 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "179")]
#[test_case(r#####"tw`placeholder-violet-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(139 92 246 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "180")]
#[test_case(r#####"tw`placeholder-violet-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(124 58 237 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "181")]
#[test_case(r#####"tw`placeholder-violet-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(109 40 217 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "182")]
#[test_case(r#####"tw`placeholder-violet-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(91 33 182 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "183")]
#[test_case(r#####"tw`placeholder-violet-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(76 29 149 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "184")]
#[test_case(r#####"tw`placeholder-purple-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(250 245 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "185")]
#[test_case(r#####"tw`placeholder-purple-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(243 232 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "186")]
#[test_case(r#####"tw`placeholder-purple-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(233 213 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "187")]
#[test_case(r#####"tw`placeholder-purple-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(216 180 254 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "188")]
#[test_case(r#####"tw`placeholder-purple-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(192 132 252 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "189")]
#[test_case(r#####"tw`placeholder-purple-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(168 85 247 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "190")]
#[test_case(r#####"tw`placeholder-purple-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(147 51 234 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "191")]
#[test_case(r#####"tw`placeholder-purple-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(126 34 206 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "192")]
#[test_case(r#####"tw`placeholder-purple-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(107 33 168 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "193")]
#[test_case(r#####"tw`placeholder-purple-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(88 28 135 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "194")]
#[test_case(r#####"tw`placeholder-fuchsia-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(253 244 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "195")]
#[test_case(r#####"tw`placeholder-fuchsia-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(250 232 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "196")]
#[test_case(r#####"tw`placeholder-fuchsia-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(245 208 254 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "197")]
#[test_case(r#####"tw`placeholder-fuchsia-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(240 171 252 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "198")]
#[test_case(r#####"tw`placeholder-fuchsia-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(232 121 249 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "199")]
#[test_case(r#####"tw`placeholder-fuchsia-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(217 70 239 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "200")]
#[test_case(r#####"tw`placeholder-fuchsia-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(192 38 211 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "201")]
#[test_case(r#####"tw`placeholder-fuchsia-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(162 28 175 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "202")]
#[test_case(r#####"tw`placeholder-fuchsia-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(134 25 143 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "203")]
#[test_case(r#####"tw`placeholder-fuchsia-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(112 26 117 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "204")]
#[test_case(r#####"tw`placeholder-pink-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(253 242 248 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "205")]
#[test_case(r#####"tw`placeholder-pink-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(252 231 243 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "206")]
#[test_case(r#####"tw`placeholder-pink-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(251 207 232 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "207")]
#[test_case(r#####"tw`placeholder-pink-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(249 168 212 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "208")]
#[test_case(r#####"tw`placeholder-pink-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(244 114 182 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "209")]
#[test_case(r#####"tw`placeholder-pink-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(236 72 153 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "210")]
#[test_case(r#####"tw`placeholder-pink-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(219 39 119 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "211")]
#[test_case(r#####"tw`placeholder-pink-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(190 24 93 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "212")]
#[test_case(r#####"tw`placeholder-pink-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(157 23 77 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "213")]
#[test_case(r#####"tw`placeholder-pink-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(131 24 67 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "214")]
#[test_case(r#####"tw`placeholder-rose-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(255 241 242 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "215")]
#[test_case(r#####"tw`placeholder-rose-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(255 228 230 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "216")]
#[test_case(r#####"tw`placeholder-rose-200`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(254 205 211 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "217")]
#[test_case(r#####"tw`placeholder-rose-300`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(253 164 175 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "218")]
#[test_case(r#####"tw`placeholder-rose-400`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(251 113 133 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "219")]
#[test_case(r#####"tw`placeholder-rose-500`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(244 63 94 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "220")]
#[test_case(r#####"tw`placeholder-rose-600`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(225 29 72 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "221")]
#[test_case(r#####"tw`placeholder-rose-700`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(190 18 60 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "222")]
#[test_case(r#####"tw`placeholder-rose-800`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(159 18 57 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "223")]
#[test_case(r#####"tw`placeholder-rose-900`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(136 19 55 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "224")]
#[test_case(r#####"tw`placeholder-[red]`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(255 0 0 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "225")]
#[test_case(r#####"tw`placeholder-[var(--placeholder)]`"#####, r#####"({
  '::placeholder': {
    color: "var(--placeholder)",
  },
})
;"##### ; "226")]
#[test_case(r#####"tw`placeholder-red-500/25`"#####, r#####"({
  '::placeholder': {
    color: "rgb(239 68 68 / 0.25)",
  },
})
;"##### ; "227")]
#[test_case(r#####"tw`placeholder-red-500/fromConfig`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(0 0 0 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "228")]
#[test_case(r#####"tw`placeholder-red-500/fromConfig/25`"#####, r#####"({
  '::placeholder': {
    color: "rgb(0 0 0 / 0.25)",
  },
})
;"##### ; "229")]
#[test_case(r#####"tw`placeholder-red-500/fromConfig/[.555]`"#####, r#####"({
  '::placeholder': {
    color: "rgb(0 0 0 / .555)",
  },
})
;"##### ; "230")]
#[test_case(r#####"tw`placeholder-red-500/fromConfig/[var(--myvar)]`"#####, r#####"({
  '::placeholder': {
    color: "rgb(0 0 0 / var(--myvar))",
  },
})
;"##### ; "231")]
#[test_case(r#####"tw`placeholder-red-500/[.555]`"#####, r#####"({
  '::placeholder': {
    color: "rgb(239 68 68 / .555)",
  },
})
;"##### ; "232")]
#[test_case(r#####"tw`placeholder-red-500/[var(--myvar)]`"#####, r#####"({
  '::placeholder': {
    color: "rgb(239 68 68 / var(--myvar))",
  },
})
;"##### ; "233")]
#[test_case(r#####"tw`placeholder-[theme('colors.red.500')]`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(239 68 68 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "234")]
#[test_case(r#####"tw`placeholder-electric`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgba(219, 0, 255, var(--tw-placeholder-opacity))",
  },
})
;"##### ; "235")]
#[test_case(r#####"tw`placeholder-electric/25`"#####, r#####"({
  '::placeholder': {
    color: "rgba(219, 0, 255, 0.25)",
  },
})
;"##### ; "236")]
#[test_case(r#####"tw`placeholder-electric/[.555]`"#####, r#####"({
  '::placeholder': {
    color: "rgba(219, 0, 255, .555)",
  },
})
;"##### ; "237")]
#[test_case(r#####"tw`placeholder-electric/[var(--myvar)]`"#####, r#####"({
  '::placeholder': {
    color: "rgba(219, 0, 255, var(--myvar))",
  },
})
;"##### ; "238")]
#[test_case(r#####"tw`placeholder-[theme('colors.electric')]`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(219 0 255 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "239")]
#[test_case(r#####"tw`placeholder-[color:red]`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(255 0 0 / var(--tw-placeholder-opacity))",
  },
})
;"##### ; "240")]
#[test_case(r#####"tw`placeholder-[any:red]`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
    color: "rgb(255 0 0 / var(--tw-placeholder-opacity))",
  },
})"##### ; "241")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
