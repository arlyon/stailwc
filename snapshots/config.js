
/**
 * Test the config matching is working correctly
 */

tw`animate-zoom-.5`

tw`text-number`
tw`text-purple`
tw`text-purple-hyphen`
tw`text-mycolors`
tw`text-mycolors-a-purple`
tw`text-mycolors-a-number`
tw`text-mycolors-array`
tw`text-my-blue-100`
tw`text-color-opacity`
tw`text-color-deep-config-500`

tw`bg-number`
tw`bg-purple`
tw`bg-purple-hyphen`
tw`bg-mycolors`
tw`bg-mycolors-a-purple`
tw`bg-mycolors-a-number`
tw`bg-mycolors-array`
tw`bg-my-blue-100`
tw`bg-color-opacity`
tw`bg-color-deep-config-500`

tw`bg-blue`
tw`bg-blue-gray`
tw`bg-blue-gray-200`
tw`bg-blue-gray-green`
tw`bg-blue-gray-green-200`
tw`bg-blue-gray-green-deep-dish`
tw`bg-blue-gray-green-deep-dish-200`
tw`bg-blue-gray-green-pink`

tw`font-customFontWeightAsString`
tw`font-customFontWeightAsNumber`

      ↓ ↓ ↓ ↓ ↓ ↓

/**
 * Test the config matching is working correctly
 */
({
  animation: 'zoom-.5 2s',
});
({
  color: '0',
});
({
  '--tw-text-opacity': '1',
  color: 'rgb(128 0 128 / var(--tw-text-opacity))',
});
({
  color: 'purple-hyphen',
});
({
  '--tw-text-opacity': '1',
  color: 'rgb(0 0 255 / var(--tw-text-opacity))',
});
({
  '--tw-text-opacity': '1',
  color: 'rgb(128 0 128 / var(--tw-text-opacity))',
});
({
  color: '0',
});
({
  color: 'blue,purple,orange',
});
({
  '--tw-text-opacity': '1',
  color: 'rgb(0 0 255 / var(--tw-text-opacity))',
});
({
  color: 'rgba(var(--color-primary), var(--tw-text-opacity, 1))',
});
({
  '--tw-text-opacity': '1',
  color: 'rgb(7 71 166 / var(--tw-text-opacity))',
});
({
  backgroundColor: '0',
});
({
  '--tw-bg-opacity': '1',
  backgroundColor: 'rgb(128 0 128 / var(--tw-bg-opacity))',
});
({
  backgroundColor: 'purple-hyphen',
});
({
  '--tw-bg-opacity': '1',
  backgroundColor: 'rgb(0 0 255 / var(--tw-bg-opacity))',
});
({
  '--tw-bg-opacity': '1',
  backgroundColor: 'rgb(128 0 128 / var(--tw-bg-opacity))',
});
({
  backgroundColor: '0',
});
({
  backgroundColor: 'blue,purple,orange',
});
({
  '--tw-bg-opacity': '1',
  backgroundColor: 'rgb(0 0 255 / var(--tw-bg-opacity))',
});
({
  backgroundColor: 'rgba(var(--color-primary), var(--tw-bg-opacity, 1))',
});
({
  '--tw-bg-opacity': '1',
  backgroundColor: 'rgb(7 71 166 / var(--tw-bg-opacity))',
});
({
  backgroundColor: 'blue-default',
});
({
  backgroundColor: 'blue-gray-default',
});
({
  backgroundColor: 'blue-gray-200',
});
({
  backgroundColor: 'blue-gray-green-default',
});
({
  backgroundColor: 'blue-gray-green-200',
});
({
  backgroundColor: 'blue-gray-green-deep-dish-default',
});
({
  backgroundColor: 'blue-gray-green-deep-dish-200',
});
({
  backgroundColor: 'blue-gray-green-pink',
});
({
  fontWeight: '700',
});
({
  fontWeight: '800',
})


