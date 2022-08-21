

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/animation
theme`animation`

tw`animate-none`
tw`animate-spin`
tw`animate-ping`
tw`animate-pulse`
tw`animate-bounce`

tw`animate-[wiggle 1s ease-in-out infinite]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/animation
({
  none: 'none',
  spin: 'spin 1s linear infinite',
  ping: 'ping 1s cubic-bezier(0, 0, 0.2, 1) infinite',
  pulse: 'pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
  bounce: 'bounce 1s infinite',
});
({
  animation: 'none',
});
({
  animation: 'spin 1s linear infinite',
});
({
  animation: 'ping 1s cubic-bezier(0, 0, 0.2, 1) infinite',
});
({
  animation: 'pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
});
({
  animation: 'bounce 1s infinite',
});
({
  animation: 'wiggle 1s ease-in-out infinite',
})


