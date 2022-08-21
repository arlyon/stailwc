

import tw from './macro' // twinImport

tw`dark:block`
tw`light:block`

      ↓ ↓ ↓ ↓ ↓ ↓

// twinImport
({
  '.test-dark &': {
    display: 'block',
  },
});
({
  '.test-light &': {
    display: 'block',
  },
})


