

import { globalStyles } from './macro'

globalStyles
;() => (
  <>
    <div tw="p-5 m-5">
      <div tw="p-5 m-5">
        <button tw="btn btn-outline">Button</button>
        <button tw="btn btn-outline btn-primary">Button</button>
        <button tw="btn btn-outline btn-secondary">Button</button>
        <button tw="btn btn-outline btn-accent">Button</button>
      </div>
      <div tw="p-5 m-5">
        <div tw="alert alert-error shadow-lg">
          <div>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              tw="stroke-current flex-shrink-0 h-6 w-6"
              fill="none"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <span>Error! Task failed successfully.</span>
          </div>
        </div>

        <div tw="alert shadow-lg">
          <div>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              tw="stroke-info flex-shrink-0 w-6 h-6"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              ></path>
            </svg>
            <span>12 unread messages. Tap to see.</span>
          </div>
        </div>
      </div>

      <div tw="p-5 m-5">
        <label htmlFor="my-modal" tw="btn" className="modal-button">
          open modal
        </label>
      </div>
      <div tw="p-5 m-5">
        <label tw="swap" className="swap">
          <input type="checkbox" />
          <div tw="swap-on">ON</div>
          <div tw="swap-off">OFF</div>
        </label>
      </div>
      <div tw="p-5 m-5">
        <div tw="badge badge-info gap-2">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            tw="inline-block w-4 h-4 stroke-current"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M6 18L18 6M6 6l12 12"
            ></path>
          </svg>
          info
        </div>
        <div tw="badge badge-success gap-2">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            tw="inline-block w-4 h-4 stroke-current"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M6 18L18 6M6 6l12 12"
            ></path>
          </svg>
          success
        </div>
        <div tw="badge badge-warning gap-2">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            tw="inline-block w-4 h-4 stroke-current"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M6 18L18 6M6 6l12 12"
            ></path>
          </svg>
          warning
        </div>
        <div tw="badge badge-error gap-2">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            tw="inline-block w-4 h-4 stroke-current"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M6 18L18 6M6 6l12 12"
            ></path>
          </svg>
          error
        </div>
      </div>
      <div tw="p-5 m-5">
        <span tw="countdown">
          <span
            css={{
              '--value': '25',
            }}
          ></span>
        </span>
      </div>
      <div tw="p-5 m-5">
        <div tw="stats shadow">
          <div tw="stat">
            <div tw="stat-figure text-primary">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                tw="inline-block w-8 h-8 stroke-current"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                ></path>
              </svg>
            </div>
            <div tw="stat-title">Total Likes</div>
            <div tw="stat-value text-primary">25.6K</div>
            <div tw="stat-desc">21% more than last month</div>
          </div>

          <div tw="stat">
            <div tw="stat-figure text-secondary">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                tw="inline-block w-8 h-8 stroke-current"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  d="M13 10V3L4 14h7v7l9-11h-7z"
                ></path>
              </svg>
            </div>
            <div tw="stat-title">Page Views</div>
            <div tw="stat-value text-secondary">2.6M</div>
            <div tw="stat-desc">21% more than last month</div>
          </div>

          <div tw="stat">
            <div tw="stat-figure text-secondary">
              <div tw="avatar" className="online">
                <div tw="w-16 rounded-full">
                  <img src="https://api.lorem.space/image/face?w=128&h=128" />
                </div>
              </div>
            </div>
            <div tw="stat-value">86%</div>
            <div tw="stat-title">Tasks done</div>
            <div tw="stat-desc text-secondary">31 tasks remaining</div>
          </div>
        </div>
      </div>
      <div tw="p-5 m-5">
        <div tw="card card-side bg-base-100 shadow-xl">
          <figure>
            <img
              src="https://api.lorem.space/image/movie?w=200&h=280"
              alt="Movie"
            />
          </figure>
          <div tw="card-body">
            <h2 tw="card-title">New movie is released!</h2>
            <p>Click the button to watch on Jetflix app.</p>
            <div tw="card-actions justify-end">
              <button tw="btn btn-primary">Watch</button>
            </div>
          </div>
        </div>
      </div>
      <div tw="p-5 m-5">
        <div className="tooltip" tw="tooltip" data-tip="hello">
          <button tw="btn">Bottom</button>
        </div>
      </div>
      <div tw="p-5 m-5">
        <div tw="form-control">
          <label tw="label cursor-pointer">
            <span tw="label-text">Remember me</span>
            <input type="checkbox" tw="checkbox checkbox-primary" />
          </label>
        </div>
      </div>
      <div tw="p-5 m-5">
        <div tw="form-control w-full max-w-xs">
          <label tw="label">
            <span tw="label-text">What is your name?</span>
            <span tw="label-text-alt">Alt label</span>
          </label>
          <input
            type="text"
            placeholder="Type here"
            tw="input input-bordered input-secondary w-full max-w-xs"
          />
          <label tw="label">
            <span tw="label-text-alt">Alt label</span>
            <span tw="label-text-alt">Alt label</span>
          </label>
        </div>
      </div>
      <div tw="p-5 m-5">
        <div tw="form-control">
          <label tw="label cursor-pointer">
            <span tw="label-text">Red pill</span>
            <input
              type="radio"
              name="radio-6"
              tw="radio checked:bg-red-500"
              // checked
            />
          </label>
        </div>
        <div tw="form-control">
          <label tw="label cursor-pointer">
            <span tw="label-text">blue pill</span>
            <input
              type="radio"
              name="radio-6"
              tw="radio checked:bg-blue-500"
              // checked
            />
          </label>
        </div>
      </div>
      <div tw="p-5 m-5">
        <input type="range" min="0" max="100" tw="range range-secondary" />
      </div>
      <div tw="p-5 m-5">
        <div tw="rating gap-1">
          <input type="radio" name="rating-3" tw="mask mask-heart bg-red-400" />
          <input
            type="radio"
            name="rating-3"
            tw="mask mask-heart bg-orange-400"
          />
          <input
            type="radio"
            name="rating-3"
            tw="mask mask-heart bg-yellow-400"
          />
          <input
            type="radio"
            name="rating-3"
            tw="mask mask-heart bg-lime-400"
          />
          <input
            type="radio"
            name="rating-3"
            tw="mask mask-heart bg-green-400"
          />
        </div>
      </div>
      <div tw="p-5 m-5">
        <select tw="select select-success w-full max-w-xs">
          <option disabled>Pick your favorite anime</option>
          <option>One Piece</option>
          <option>Naruto</option>
          <option>Death Note</option>
          <option>Attack on Titan</option>
          <option>Bleach</option>
          <option>Fullmetal Alchemist</option>
          <option>Jojo's Bizarre Adventure</option>
        </select>
      </div>
      <div tw="p-5 m-5">
        <input type="checkbox" tw="toggle" />
      </div>
      <div tw="p-5 m-5">
        <div tw="btn-group">
          <input
            type="radio"
            name="options"
            data-title="1"
            tw="btn"
            className="btn"
          />
          <input
            type="radio"
            name="options"
            data-title="2"
            tw="btn"
            className="btn"
          />
          <input
            type="radio"
            name="options"
            data-title="3"
            tw="btn"
            className="btn"
          />
          <input
            type="radio"
            name="options"
            data-title="4"
            tw="btn"
            className="btn"
          />
        </div>
      </div>
      <div tw="p-5 m-5">
        <div tw="drawer" className="drawer">
          <input id="my-drawer" type="checkbox" tw="drawer-toggle" />
          <div tw="drawer-content" className="drawer-content">
            <label htmlFor="my-drawer" tw="btn btn-primary drawer-button">
              Open drawer
            </label>
          </div>
          <div tw="drawer-side" className="drawer-side">
            <label
              htmlFor="my-drawer"
              tw="drawer-overlay"
              className="drawer-overlay"
            ></label>
            <ul
              className="menu"
              tw="menu p-4 overflow-y-auto w-80 bg-base-100 text-base-content"
            >
              <li>
                <a>Sidebar Item 1</a>
              </li>
              <li>
                <a>Sidebar Item 2</a>
              </li>
            </ul>
          </div>
        </div>
      </div>
      <div tw="p-5 m-5">
        <div tw="hero min-h-screen bg-base-200">
          <div tw="hero-content text-center">
            <div tw="max-w-md">
              <h1 tw="text-5xl font-bold">Hello there</h1>
              <p tw="py-6">
                Provident cupiditate voluptatem et in. Quaerat fugiat ut
                assumenda excepturi exercitationem quasi. In deleniti eaque aut
                repudiandae et a id nisi.
              </p>
              <button tw="btn btn-primary">Get Started</button>
            </div>
          </div>
        </div>
      </div>
      <div tw="p-5 m-5">
        <div tw="indicator" className="indicator">
          <span
            tw="indicator-item badge badge-secondary"
            className="indicator-item"
          >
            99+
          </span>
          <button tw="btn">inbox</button>
        </div>
      </div>
      <div tw="p-5 m-5">
        <div tw="text-sm breadcrumbs">
          <ul>
            <li>
              <a>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  tw="w-4 h-4 mr-2 stroke-current"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
                  ></path>
                </svg>
                Home
              </a>
            </li>
            <li>
              <a>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  tw="w-4 h-4 mr-2 stroke-current"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
                  ></path>
                </svg>
                Documents
              </a>
            </li>
            <li>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                tw="w-4 h-4 mr-2 stroke-current"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  d="M9 13h6m-3-3v6m5 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                ></path>
              </svg>
              Add Document
            </li>
          </ul>
        </div>
      </div>
      <div tw="p-5 m-5">
        <ul tw="steps" className="steps">
          <li className="step step-primary" tw=" step-primary">
            Register
          </li>
          <li className="step step-primary" tw=" step-primary">
            Choose plan
          </li>
          <li className="step  step-primary" tw=" step-primary">
            Purchase
          </li>
          <li className="step" tw="">
            Receive Product
          </li>
        </ul>
      </div>
      <div tw="p-5 m-5">
        <ul tw="steps" className="steps">
          <li className="step step-info" tw="step step-info">
            Fly to moon
          </li>
          <li className="step step-info" tw="step step-info">
            Shrink the moon
          </li>
          <li className="step step-info" tw="step step-info">
            Grab the moon
          </li>
          <li className="step step-error" tw="step step-error" data-content="?">
            Sit on toilet
          </li>
        </ul>
      </div>
      <div tw="p-5 m-5">
        <div tw="tabs">
          <a tw="tab tab-lg tab-lifted">Tab 1</a>
          <a tw="tab tab-lg tab-lifted" className=" tab-active">
            Tab 2
          </a>
          <a tw="tab tab-lg tab-lifted">Tab 3</a>
        </div>
      </div>
      <div tw="p-5 m-5">
        <div tw="mockup-code">
          <pre data-prefix="$">
            <code>npm i daisyui</code>
          </pre>
          <pre data-prefix=">" tw="text-warning">
            <code>installing...</code>
          </pre>
          <pre data-prefix=">" tw="text-success">
            <code>Done!</code>
          </pre>
        </div>
      </div>
      <div tw="p-5 m-5">
        <div tw="card w-96 bg-base-100 shadow-xl">
          <figure>
            <img
              src="https://api.lorem.space/image/shoes?w=400&h=225"
              alt="Shoes"
            />
          </figure>
          <div tw="card-body">
            <h2 tw="card-title">Shoes!</h2>
            <p>If a dog chews shoes whose shoes does he choose?</p>
            <div tw="card-actions justify-end">
              <button tw="btn btn-primary">Buy Now</button>
            </div>
          </div>
        </div>
      </div>
      <div tw="p-5 m-5 gap-2 grid">
        <div tw="carousel w-full">
          <div id="item1" tw="carousel-item w-full">
            <img
              src="https://api.lorem.space/image/car?w=800&h=200&hash=8B7BCDC2"
              tw="w-full"
            />
          </div>
          <div id="item2" tw="carousel-item w-full">
            <img
              src="https://api.lorem.space/image/car?w=800&h=200&hash=500B67FB"
              tw="w-full"
            />
          </div>
          <div id="item3" tw="carousel-item w-full">
            <img
              src="https://api.lorem.space/image/car?w=800&h=200&hash=A89D0DE6"
              tw="w-full"
            />
          </div>
          <div id="item4" tw="carousel-item w-full">
            <img
              src="https://api.lorem.space/image/car?w=800&h=200&hash=225E6693"
              tw="w-full"
            />
          </div>
        </div>
        <div tw="flex justify-center w-full py-2 gap-2">
          <a href="#item1" tw="btn btn-xs">
            1
          </a>
          <a href="#item2" tw="btn btn-xs">
            2
          </a>
          <a href="#item3" tw="btn btn-xs">
            3
          </a>
          <a href="#item4" tw="btn btn-xs">
            4
          </a>
        </div>
      </div>
      <div tw="p-5 m-5">
        <div tw="indicator" className="indicator">
          <span
            className="indicator-item"
            tw="indicator-item indicator-top indicator-start badge badge-secondary"
          >
            top+start
          </span>
          <span
            className="indicator-item indicator-center"
            tw="indicator-item indicator-top indicator-center badge badge-secondary"
          >
            top+center
          </span>
          <span
            className="indicator-item"
            tw="indicator-item indicator-top indicator-end badge badge-secondary"
          >
            top+end
          </span>
          <span
            className="indicator-item"
            tw="indicator-item indicator-middle indicator-start badge badge-secondary"
          >
            middle+start
          </span>
          <span
            className="indicator-item"
            tw="indicator-item indicator-middle indicator-center badge badge-secondary"
          >
            middle+center
          </span>
          <span
            className="indicator-item"
            tw="indicator-item indicator-middle indicator-end badge badge-secondary"
          >
            middle+end
          </span>
          <span
            className="indicator-item"
            tw="indicator-item indicator-bottom indicator-start badge badge-secondary"
          >
            bott0m+strt
          </span>
          <span
            className="indicator-item"
            tw="indicator-item indicator-bottom indicator-center badge badge-secondary"
          >
            bottom+center
          </span>
          <span
            className="indicator-item"
            tw="indicator-item indicator-bottom indicator-end badge badge-secondary"
          >
            bottom+end
          </span>
          <div tw="grid w-60 h-32 bg-base-300 place-items-center">content</div>
        </div>
      </div>
      <div tw="p-5 m-5">
        <div tw="hero min-h-screen bg-base-200">
          <div tw="hero-content flex-col lg:flex-row-reverse">
            <div tw="text-center lg:text-left">
              <h1 tw="text-5xl font-bold">Login now!</h1>
              <p tw="py-6">
                Provident cupiditate voluptatem et in. Quaerat fugiat ut
                assumenda excepturi exercitationem quasi. In deleniti eaque aut
                repudiandae et a id nisi.
              </p>
            </div>
            <div tw="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
              <div tw="card-body">
                <div tw="form-control">
                  <label tw="label">
                    <span tw="label-text">Email</span>
                  </label>
                  <input
                    type="text"
                    placeholder="email"
                    tw="input input-bordered"
                  />
                </div>
                <div tw="form-control">
                  <label tw="label">
                    <span tw="label-text">Password</span>
                  </label>
                  <input
                    type="text"
                    placeholder="password"
                    tw="input input-bordered"
                  />
                  <label tw="label">
                    <a href="#" tw="label-text-alt link link-hover">
                      Forgot password?
                    </a>
                  </label>
                </div>
                <div tw="form-control mt-6">
                  <button tw="btn btn-primary">Login</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <input
      type="checkbox"
      id="my-modal"
      tw="modal-toggle"
      className="modal-toggle"
    />
    <div tw="modal" className="modal">
      <div tw="modal-box">
        <h3 tw="font-bold text-lg">Congratulations random Interner user!</h3>
        <p tw="py-4">
          You've been selected for a chance to get one year of subscription to
          use Wikipedia for free!
        </p>
        <div tw="modal-action" className="modal-action">
          <label htmlFor="my-modal" tw="btn">
            Yay!d
          </label>
        </div>
      </div>
    </div>
  </>
)

      ↓ ↓ ↓ ↓ ↓ ↓

({
  '*, ::before, ::after': {
    boxSizing: 'border-box',
    borderWidth: '0',
    borderStyle: 'solid',
    borderColor: '#e5e7eb',
    '--tw-translate-x': '0',
    '--tw-translate-y': '0',
    '--tw-rotate': '0',
    '--tw-skew-x': '0',
    '--tw-skew-y': '0',
    '--tw-scale-x': '1',
    '--tw-scale-y': '1',
    '--tw-pan-x': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-pan-y': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-pinch-zoom': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-scroll-snap-strictness': 'proximity',
    '--tw-ordinal': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-slashed-zero': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-numeric-figure': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-numeric-spacing': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-numeric-fraction': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-ring-inset': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-ring-offset-width': '0px',
    '--tw-ring-offset-color': '#fff',
    '--tw-ring-color': 'rgb(59 130 246 / 0.5)',
    '--tw-ring-offset-shadow': '0 0 #0000',
    '--tw-ring-shadow': '0 0 #0000',
    '--tw-shadow': '0 0 #0000',
    '--tw-shadow-colored': '0 0 #0000',
    '--tw-blur': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-brightness': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-contrast': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-grayscale': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-hue-rotate': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-invert': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-saturate': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-sepia': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-drop-shadow': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-blur': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-brightness': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-contrast': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-grayscale': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-hue-rotate': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-invert': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-opacity': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-saturate': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-sepia': 'var(--tw-empty,/*!*/ /*!*/)',
  },
  '::before, ::after': {
    '--tw-content': "''",
  },
  html: {
    lineHeight: '1.5',
    WebkitTextSizeAdjust: '100%',
    MozTabSize: '4',
    tabSize: '4',
    fontFamily:
      'ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"',
    WebkitTapHighlightColor: 'transparent',
  },
  body: {
    margin: '0',
    lineHeight: 'inherit',
  },
  hr: {
    height: '0',
    color: 'inherit',
    borderTopWidth: '1px',
  },
  'abbr:where([title])': {
    textDecoration: 'underline dotted',
  },
  'h1, h2, h3, h4, h5, h6': {
    fontSize: 'inherit',
    fontWeight: 'inherit',
  },
  a: {
    color: 'inherit',
    textDecoration: 'inherit',
  },
  'b, strong': {
    fontWeight: 'bolder',
  },
  'code, kbd, samp, pre': {
    fontFamily:
      'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
    fontSize: '1em',
  },
  small: {
    fontSize: '80%',
  },
  'sub, sup': {
    fontSize: '75%',
    lineHeight: '0',
    position: 'relative',
    verticalAlign: 'baseline',
  },
  sub: {
    bottom: '-0.25em',
  },
  sup: {
    top: '-0.5em',
  },
  table: {
    textIndent: '0',
    borderColor: 'inherit',
    borderCollapse: 'collapse',
  },
  'button, input, optgroup, select, textarea': {
    fontFamily: 'inherit',
    fontSize: '100%',
    lineHeight: 'inherit',
    color: 'inherit',
    margin: '0',
    padding: '0',
  },
  'button, select': {
    textTransform: 'none',
  },
  'button, [type="button"], [type="reset"], [type="submit"]': {
    WebkitAppearance: 'button',
    backgroundColor: 'transparent',
    backgroundImage: 'none',
  },
  ':-moz-focusring': {
    outline: 'auto',
  },
  ':-moz-ui-invalid': {
    boxShadow: 'none',
  },
  progress: {
    verticalAlign: 'baseline',
  },
  '::-webkit-inner-spin-button, ::-webkit-outer-spin-button': {
    height: 'auto',
  },
  '[type="search"]': {
    WebkitAppearance: 'textfield',
    outlineOffset: '-2px',
  },
  '::-webkit-search-decoration': {
    WebkitAppearance: 'none',
  },
  '::-webkit-file-upload-button': {
    WebkitAppearance: 'button',
    font: 'inherit',
  },
  summary: {
    display: 'list-item',
  },
  'blockquote, dl, dd, h1, h2, h3, h4, h5, h6, hr, figure, p, pre': {
    margin: '0',
  },
  fieldset: {
    margin: '0',
    padding: '0',
  },
  legend: {
    padding: '0',
  },
  'ol, ul, menu': {
    listStyle: 'none',
    margin: '0',
    padding: '0',
  },
  textarea: {
    resize: 'vertical',
  },
  'input::-moz-placeholder, textarea::-moz-placeholder': {
    opacity: '1',
    color: '#9ca3af',
  },
  'input:-ms-input-placeholder, textarea:-ms-input-placeholder': {
    opacity: '1',
    color: '#9ca3af',
  },
  'input::placeholder, textarea::placeholder': {
    opacity: '1',
    color: '#9ca3af',
  },
  'button, [role="button"]': {
    cursor: 'pointer',
  },
  ':disabled, [disabled]': {
    cursor: 'default',
  },
  'img, svg, video, canvas, audio, iframe, embed, object': {
    display: 'block',
    verticalAlign: 'middle',
  },
  'img, video': {
    maxWidth: '100%',
    height: 'auto',
  },
  '[hidden]': {
    display: 'none',
  },
  '@keyframes spin': {
    to: {
      transform: 'rotate(360deg)',
    },
    from: {
      transform: 'rotate(0deg)',
    },
  },
  '@keyframes ping': {
    '75%, 100%': {
      transform: 'scale(2)',
      opacity: '0',
    },
  },
  '@keyframes pulse': {
    '50%': {
      opacity: '.5',
    },
  },
  '@keyframes bounce': {
    '0%, 100%': {
      transform: 'translateY(-25%)',
      animationTimingFunction: 'cubic-bezier(0.8,0,1,1)',
    },
    '50%': {
      transform: 'none',
      animationTimingFunction: 'cubic-bezier(0,0,0.2,1)',
    },
  },
  ':root, [data-theme]': {
    backgroundColor: 'hsla(var(--b1) / var(--tw-bg-opacity, 1))',
    color: 'hsla(var(--bc) / var(--tw-text-opacity, 1))',
  },
  ':root': {
    '--p': '259 94% 51%',
    '--pf': '259 94% 41%',
    '--sf': '314 100% 38%',
    '--af': '174 60% 41%',
    '--nf': '219 14% 22%',
    '--in': '198 93% 60%',
    '--su': '158 64% 52%',
    '--wa': '43 96% 56%',
    '--er': '0 91% 71%',
    '--inc': '198 100% 12%',
    '--suc': '158 100% 10%',
    '--wac': '43 100% 11%',
    '--erc': '0 100% 14%',
    '--rounded-box': '1rem',
    '--rounded-btn': '0.5rem',
    '--rounded-badge': '1.9rem',
    '--animation-btn': '0.25s',
    '--animation-input': '.2s',
    '--btn-text-case': 'uppercase',
    '--btn-focus-scale': '0.95',
    '--border-btn': '1px',
    '--tab-border': '1px',
    '--tab-radius': '0.5rem',
    '--pc': '0 0% 100%',
    '--s': '314 100% 47%',
    '--sc': '0 0% 100%',
    '--a': '174 60% 51%',
    '--ac': '175 44% 15%',
    '--n': '219 14% 28%',
    '--nc': '0 0% 100%',
    '--b1': '0 0% 100%',
    '--b2': '0 0% 95%',
    '--b3': '180 2% 90%',
    '--bc': '215 28% 17%',
  },
  '@keyframes button-pop': {
    '0%': {
      transform: 'scale(var(--btn-focus-scale, 0.95))',
    },
    '40%': {
      transform: 'scale(1.02)',
    },
    '100%': {
      transform: 'scale(1)',
    },
  },
  '@keyframes checkmark': {
    '0%': {
      backgroundPositionY: '5px',
    },
    '50%': {
      backgroundPositionY: '-2px',
    },
    '100%': {
      backgroundPositionY: '0',
    },
  },
  '@keyframes progress-loading': {
    '50%': {
      left: '107%',
    },
  },
  '@keyframes radiomark': {
    '0%': {
      boxShadow:
        '0 0 0 12px hsl(var(--b1)) inset, 0 0 0 12px hsl(var(--b1)) inset',
    },
    '50%': {
      boxShadow:
        '0 0 0 3px hsl(var(--b1)) inset, 0 0 0 3px hsl(var(--b1)) inset',
    },
    '100%': {
      boxShadow:
        '0 0 0 4px hsl(var(--b1)) inset, 0 0 0 4px hsl(var(--b1)) inset',
    },
  },
  '@keyframes rating-pop': {
    '0%': {
      transform: 'translateY(-0.125em)',
    },
    '40%': {
      transform: 'translateY(-0.125em)',
    },
    '100%': {
      transform: 'translateY(0)',
    },
  },
})

;() => (
  <>
    <div
      css={{
        padding: '1.25rem',
        margin: '1.25rem',
      }}
    >
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <button
          css={{
            display: 'inline-flex',
            flexShrink: '0',
            cursor: 'pointer',
            userSelect: 'none',
            flexWrap: 'wrap',
            alignItems: 'center',
            justifyContent: 'center',
            borderColor: 'currentColor',
            textAlign: 'center',
            transitionProperty:
              'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
            transitionDuration: '200ms',
            transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
            borderRadius: 'var(--rounded-btn, 0.5rem)',
            height: '3rem',
            paddingLeft: '1rem',
            paddingRight: '1rem',
            fontSize: '0.875rem',
            lineHeight: '1em',
            minHeight: '3rem',
            fontWeight: '600',
            textTransform: 'var(--btn-text-case, uppercase)',
            textDecorationLine: 'none',
            borderWidth: 'var(--border-btn, 1px)',
            animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
            '--tw-border-opacity': '1',
            '--tw-bg-opacity': '1',
            backgroundColor: 'transparent',
            '--tw-text-opacity': '1',
            color: 'hsl(var(--bc) / var(--tw-text-opacity))',
            '&.loading': {
              pointerEvents: 'none',
            },
            '&.loading:hover': {
              pointerEvents: 'none',
            },
            '&.loading:before': {
              marginRight: '0.5rem',
              height: '1rem',
              width: '1rem',
              borderRadius: '9999px',
              borderWidth: '2px',
              animation: 'spin 2s linear infinite',
              content: '""',
              borderTopColor: 'transparent',
              borderLeftColor: 'transparent',
              borderBottomColor: 'currentColor',
              borderRightColor: 'currentColor',
            },
            ':active:hover': {
              animation: 'none',
              transform: 'scale(var(--btn-focus-scale, 0.95))',
            },
            ':active:focus': {
              animation: 'none',
              transform: 'scale(var(--btn-focus-scale, 0.95))',
            },
            ':hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--bc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--bc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--b1) / var(--tw-text-opacity))',
            },
            ':focus-visible': {
              outline: '2px solid hsl(var(--nf))',
              outlineOffset: '2px',
            },
            '&.glass:hover': {
              '--glass-opacity': '25%',
              '--glass-border-opacity': '15%',
            },
            '&.glass.btn-active': {
              '--glass-opacity': '25%',
              '--glass-border-opacity': '15%',
            },
            '&.glass:focus-visible': {
              outline: '2px solid 0 0 2px currentColor',
            },
            '&.loading.btn-square:before': {
              marginRight: '0px',
            },
            '&.loading.btn-circle:before': {
              marginRight: '0px',
            },
            '&.loading.btn-xl:before': {
              height: '1.25rem',
              width: '1.25rem',
            },
            '&.loading.btn-lg:before': {
              height: '1.25rem',
              width: '1.25rem',
            },
            '&.loading.btn-sm:before': {
              height: '0.75rem',
              width: '0.75rem',
            },
            '&.loading.btn-xs:before': {
              height: '0.75rem',
              width: '0.75rem',
            },
            '.btn-group > input[type="radio"]:checked&': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-group > input[type="radio"]:checked&:focus-visible': {
              outline: '2px solid hsl(var(--p))',
            },
            '.btn-group > &:not(:first-of-type)': {
              marginLeft: '-1px',
              borderTopLeftRadius: '0px',
              borderBottomLeftRadius: '0px',
            },
            '.btn-group > &:not(:last-of-type)': {
              borderTopRightRadius: '0px',
              borderBottomRightRadius: '0px',
            },
            '@media (prefers-reduced-motion: reduce)': {
              '&.loading:before': {
                animation: 'spin 10s linear infinite',
              },
            },
            '& .badge': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '&.btn-primary .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-secondary .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-accent .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '& .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
            },
            '&:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--bc) / var(--tw-text-opacity))',
            },
            '&:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '&.btn-primary:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '&.btn-primary:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-secondary:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '&.btn-secondary:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-accent:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '&.btn-accent:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '&.btn-primary .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '&.btn-secondary .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '&.btn-accent .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '&.btn-info .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--in) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--in) / var(--tw-text-opacity))',
            },
            '&.btn-success .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--su) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--su) / var(--tw-text-opacity))',
            },
            '&.btn-warning .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--wa) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wa) / var(--tw-text-opacity))',
            },
            '&.btn-error .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--er) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--er) / var(--tw-text-opacity))',
            },
            '&.btn-primary': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '&.btn-primary:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-secondary': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '&.btn-secondary:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--sf, var(--s)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-accent': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '&.btn-accent:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--af, var(--a)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '&.btn-success': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--su) / var(--tw-text-opacity))',
            },
            '&.btn-success:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--su) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--su) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--suc, var(--nc)) / var(--tw-text-opacity))',
            },
            '&.btn-info': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--in) / var(--tw-text-opacity))',
            },
            '&.btn-info:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--in) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--inc, var(--nc)) / var(--tw-text-opacity))',
            },
            '&.btn-warning': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wa) / var(--tw-text-opacity))',
            },
            '&.btn-warning:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--wa) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--wa) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wac, var(--nc)) / var(--tw-text-opacity))',
            },
            '&.btn-error': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--er) / var(--tw-text-opacity))',
            },
            '&.btn-error:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--er) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--er) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--erc, var(--nc)) / var(--tw-text-opacity))',
            },
          }}
        >
          Button
        </button>
        <button
          css={{
            display: 'inline-flex',
            flexShrink: '0',
            cursor: 'pointer',
            userSelect: 'none',
            flexWrap: 'wrap',
            alignItems: 'center',
            justifyContent: 'center',
            borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
            textAlign: 'center',
            transitionProperty:
              'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
            transitionDuration: '200ms',
            transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
            borderRadius: 'var(--rounded-btn, 0.5rem)',
            height: '3rem',
            paddingLeft: '1rem',
            paddingRight: '1rem',
            fontSize: '0.875rem',
            lineHeight: '1em',
            minHeight: '3rem',
            fontWeight: '600',
            textTransform: 'var(--btn-text-case, uppercase)',
            textDecorationLine: 'none',
            borderWidth: 'var(--border-btn, 1px)',
            animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
            '--tw-border-opacity': '1',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
            '--tw-text-opacity': '1',
            color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            '&.loading': {
              pointerEvents: 'none',
            },
            '&.loading:hover': {
              pointerEvents: 'none',
            },
            '&.loading:before': {
              marginRight: '0.5rem',
              height: '1rem',
              width: '1rem',
              borderRadius: '9999px',
              borderWidth: '2px',
              animation: 'spin 2s linear infinite',
              content: '""',
              borderTopColor: 'transparent',
              borderLeftColor: 'transparent',
              borderBottomColor: 'currentColor',
              borderRightColor: 'currentColor',
            },
            ':active:hover': {
              animation: 'none',
              transform: 'scale(var(--btn-focus-scale, 0.95))',
            },
            ':active:focus': {
              animation: 'none',
              transform: 'scale(var(--btn-focus-scale, 0.95))',
            },
            ':hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--b1) / var(--tw-text-opacity))',
            },
            ':focus-visible': {
              outline: '2px solid hsl(var(--p))',
              outlineOffset: '2px',
            },
            '&.glass:hover': {
              '--glass-opacity': '25%',
              '--glass-border-opacity': '15%',
            },
            '&.glass.btn-active': {
              '--glass-opacity': '25%',
              '--glass-border-opacity': '15%',
            },
            '&.glass:focus-visible': {
              outline: '2px solid 0 0 2px currentColor',
            },
            '&.loading.btn-square:before': {
              marginRight: '0px',
            },
            '&.loading.btn-circle:before': {
              marginRight: '0px',
            },
            '&.loading.btn-xl:before': {
              height: '1.25rem',
              width: '1.25rem',
            },
            '&.loading.btn-lg:before': {
              height: '1.25rem',
              width: '1.25rem',
            },
            '&.loading.btn-sm:before': {
              height: '0.75rem',
              width: '0.75rem',
            },
            '&.loading.btn-xs:before': {
              height: '0.75rem',
              width: '0.75rem',
            },
            '.btn-group > input[type="radio"]:checked&': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-group > input[type="radio"]:checked&:focus-visible': {
              outline: '2px solid hsl(var(--p))',
            },
            '.btn-group > &:not(:first-of-type)': {
              marginLeft: '-1px',
              borderTopLeftRadius: '0px',
              borderBottomLeftRadius: '0px',
            },
            '.btn-group > &:not(:last-of-type)': {
              borderTopRightRadius: '0px',
              borderBottomRightRadius: '0px',
            },
            '@media (prefers-reduced-motion: reduce)': {
              '&.loading:before': {
                animation: 'spin 10s linear infinite',
              },
            },
            '& .badge': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '&.btn-primary .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-secondary .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-accent .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '& .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
            },
            '&:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--bc) / var(--tw-text-opacity))',
            },
            '&:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '&.btn-primary:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '&.btn-primary:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-secondary:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '&.btn-secondary:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-accent:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '&.btn-accent:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '&.btn-primary .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '&.btn-secondary .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '&.btn-accent .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '&.btn-info .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--in) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--in) / var(--tw-text-opacity))',
            },
            '&.btn-success .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--su) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--su) / var(--tw-text-opacity))',
            },
            '&.btn-warning .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--wa) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wa) / var(--tw-text-opacity))',
            },
            '&.btn-error .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--er) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--er) / var(--tw-text-opacity))',
            },
            '&.btn-primary': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '&.btn-primary:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-secondary': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '&.btn-secondary:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--sf, var(--s)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-accent': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '&.btn-accent:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--af, var(--a)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '&.btn-success': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--su) / var(--tw-text-opacity))',
            },
            '&.btn-success:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--su) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--su) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--suc, var(--nc)) / var(--tw-text-opacity))',
            },
            '&.btn-info': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--in) / var(--tw-text-opacity))',
            },
            '&.btn-info:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--in) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--inc, var(--nc)) / var(--tw-text-opacity))',
            },
            '&.btn-warning': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wa) / var(--tw-text-opacity))',
            },
            '&.btn-warning:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--wa) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--wa) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wac, var(--nc)) / var(--tw-text-opacity))',
            },
            '&.btn-error': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--er) / var(--tw-text-opacity))',
            },
            '&.btn-error:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--er) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--er) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--erc, var(--nc)) / var(--tw-text-opacity))',
            },
            '.btn-outline& .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-outline&:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '.btn-outline&:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.drawer-toggle:focus-visible ~ .drawer-content .drawer-button&': {
              outline: '2px solid hsl(var(--p))',
            },
            '.btn-outline& .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '.btn-outline&': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '.btn-outline&:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-active': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
            },
          }}
        >
          Button
        </button>
        <button
          css={{
            display: 'inline-flex',
            flexShrink: '0',
            cursor: 'pointer',
            userSelect: 'none',
            flexWrap: 'wrap',
            alignItems: 'center',
            justifyContent: 'center',
            borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
            textAlign: 'center',
            transitionProperty:
              'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
            transitionDuration: '200ms',
            transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
            borderRadius: 'var(--rounded-btn, 0.5rem)',
            height: '3rem',
            paddingLeft: '1rem',
            paddingRight: '1rem',
            fontSize: '0.875rem',
            lineHeight: '1em',
            minHeight: '3rem',
            fontWeight: '600',
            textTransform: 'var(--btn-text-case, uppercase)',
            textDecorationLine: 'none',
            borderWidth: 'var(--border-btn, 1px)',
            animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
            '--tw-border-opacity': '1',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
            '--tw-text-opacity': '1',
            color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            '&.loading': {
              pointerEvents: 'none',
            },
            '&.loading:hover': {
              pointerEvents: 'none',
            },
            '&.loading:before': {
              marginRight: '0.5rem',
              height: '1rem',
              width: '1rem',
              borderRadius: '9999px',
              borderWidth: '2px',
              animation: 'spin 2s linear infinite',
              content: '""',
              borderTopColor: 'transparent',
              borderLeftColor: 'transparent',
              borderBottomColor: 'currentColor',
              borderRightColor: 'currentColor',
            },
            ':active:hover': {
              animation: 'none',
              transform: 'scale(var(--btn-focus-scale, 0.95))',
            },
            ':active:focus': {
              animation: 'none',
              transform: 'scale(var(--btn-focus-scale, 0.95))',
            },
            ':hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--sf, var(--s)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--b1) / var(--tw-text-opacity))',
            },
            ':focus-visible': {
              outline: '2px solid hsl(var(--s))',
              outlineOffset: '2px',
            },
            '&.glass:hover': {
              '--glass-opacity': '25%',
              '--glass-border-opacity': '15%',
            },
            '&.glass.btn-active': {
              '--glass-opacity': '25%',
              '--glass-border-opacity': '15%',
            },
            '&.glass:focus-visible': {
              outline: '2px solid 0 0 2px currentColor',
            },
            '&.loading.btn-square:before': {
              marginRight: '0px',
            },
            '&.loading.btn-circle:before': {
              marginRight: '0px',
            },
            '&.loading.btn-xl:before': {
              height: '1.25rem',
              width: '1.25rem',
            },
            '&.loading.btn-lg:before': {
              height: '1.25rem',
              width: '1.25rem',
            },
            '&.loading.btn-sm:before': {
              height: '0.75rem',
              width: '0.75rem',
            },
            '&.loading.btn-xs:before': {
              height: '0.75rem',
              width: '0.75rem',
            },
            '.btn-group > input[type="radio"]:checked&': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-group > input[type="radio"]:checked&:focus-visible': {
              outline: '2px solid hsl(var(--p))',
            },
            '.btn-group > &:not(:first-of-type)': {
              marginLeft: '-1px',
              borderTopLeftRadius: '0px',
              borderBottomLeftRadius: '0px',
            },
            '.btn-group > &:not(:last-of-type)': {
              borderTopRightRadius: '0px',
              borderBottomRightRadius: '0px',
            },
            '@media (prefers-reduced-motion: reduce)': {
              '&.loading:before': {
                animation: 'spin 10s linear infinite',
              },
            },
            '& .badge': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '&.btn-primary .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-secondary .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-accent .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '& .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
            },
            '&:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--bc) / var(--tw-text-opacity))',
            },
            '&:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '&.btn-primary:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '&.btn-primary:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-secondary:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '&.btn-secondary:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-accent:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '&.btn-accent:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '&.btn-primary .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '&.btn-secondary .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '&.btn-accent .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '&.btn-info .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--in) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--in) / var(--tw-text-opacity))',
            },
            '&.btn-success .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--su) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--su) / var(--tw-text-opacity))',
            },
            '&.btn-warning .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--wa) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wa) / var(--tw-text-opacity))',
            },
            '&.btn-error .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--er) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--er) / var(--tw-text-opacity))',
            },
            '&.btn-primary': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '&.btn-primary:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-secondary': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '&.btn-secondary:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--sf, var(--s)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-accent': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '&.btn-accent:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--af, var(--a)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '&.btn-success': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--su) / var(--tw-text-opacity))',
            },
            '&.btn-success:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--su) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--su) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--suc, var(--nc)) / var(--tw-text-opacity))',
            },
            '&.btn-info': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--in) / var(--tw-text-opacity))',
            },
            '&.btn-info:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--in) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--inc, var(--nc)) / var(--tw-text-opacity))',
            },
            '&.btn-warning': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wa) / var(--tw-text-opacity))',
            },
            '&.btn-warning:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--wa) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--wa) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wac, var(--nc)) / var(--tw-text-opacity))',
            },
            '&.btn-error': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--er) / var(--tw-text-opacity))',
            },
            '&.btn-error:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--er) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--er) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--erc, var(--nc)) / var(--tw-text-opacity))',
            },
            '.btn-outline& .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '.btn-outline&:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '.btn-outline&:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '.drawer-toggle:focus-visible ~ .drawer-content .drawer-button&': {
              outline: '2px solid hsl(var(--s))',
            },
            '.btn-outline& .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '.btn-outline&': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '.btn-outline&:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--sf, var(--s)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-active': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--sf, var(--s)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
            },
          }}
        >
          Button
        </button>
        <button
          css={{
            display: 'inline-flex',
            flexShrink: '0',
            cursor: 'pointer',
            userSelect: 'none',
            flexWrap: 'wrap',
            alignItems: 'center',
            justifyContent: 'center',
            borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
            textAlign: 'center',
            transitionProperty:
              'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
            transitionDuration: '200ms',
            transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
            borderRadius: 'var(--rounded-btn, 0.5rem)',
            height: '3rem',
            paddingLeft: '1rem',
            paddingRight: '1rem',
            fontSize: '0.875rem',
            lineHeight: '1em',
            minHeight: '3rem',
            fontWeight: '600',
            textTransform: 'var(--btn-text-case, uppercase)',
            textDecorationLine: 'none',
            borderWidth: 'var(--border-btn, 1px)',
            animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
            '--tw-border-opacity': '1',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
            '--tw-text-opacity': '1',
            color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            '&.loading': {
              pointerEvents: 'none',
            },
            '&.loading:hover': {
              pointerEvents: 'none',
            },
            '&.loading:before': {
              marginRight: '0.5rem',
              height: '1rem',
              width: '1rem',
              borderRadius: '9999px',
              borderWidth: '2px',
              animation: 'spin 2s linear infinite',
              content: '""',
              borderTopColor: 'transparent',
              borderLeftColor: 'transparent',
              borderBottomColor: 'currentColor',
              borderRightColor: 'currentColor',
            },
            ':active:hover': {
              animation: 'none',
              transform: 'scale(var(--btn-focus-scale, 0.95))',
            },
            ':active:focus': {
              animation: 'none',
              transform: 'scale(var(--btn-focus-scale, 0.95))',
            },
            ':hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--af, var(--a)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--b1) / var(--tw-text-opacity))',
            },
            ':focus-visible': {
              outline: '2px solid hsl(var(--a))',
              outlineOffset: '2px',
            },
            '&.glass:hover': {
              '--glass-opacity': '25%',
              '--glass-border-opacity': '15%',
            },
            '&.glass.btn-active': {
              '--glass-opacity': '25%',
              '--glass-border-opacity': '15%',
            },
            '&.glass:focus-visible': {
              outline: '2px solid 0 0 2px currentColor',
            },
            '&.loading.btn-square:before': {
              marginRight: '0px',
            },
            '&.loading.btn-circle:before': {
              marginRight: '0px',
            },
            '&.loading.btn-xl:before': {
              height: '1.25rem',
              width: '1.25rem',
            },
            '&.loading.btn-lg:before': {
              height: '1.25rem',
              width: '1.25rem',
            },
            '&.loading.btn-sm:before': {
              height: '0.75rem',
              width: '0.75rem',
            },
            '&.loading.btn-xs:before': {
              height: '0.75rem',
              width: '0.75rem',
            },
            '.btn-group > input[type="radio"]:checked&': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-group > input[type="radio"]:checked&:focus-visible': {
              outline: '2px solid hsl(var(--p))',
            },
            '.btn-group > &:not(:first-of-type)': {
              marginLeft: '-1px',
              borderTopLeftRadius: '0px',
              borderBottomLeftRadius: '0px',
            },
            '.btn-group > &:not(:last-of-type)': {
              borderTopRightRadius: '0px',
              borderBottomRightRadius: '0px',
            },
            '@media (prefers-reduced-motion: reduce)': {
              '&.loading:before': {
                animation: 'spin 10s linear infinite',
              },
            },
            '& .badge': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '&.btn-primary .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-secondary .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-accent .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '& .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
            },
            '&:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--bc) / var(--tw-text-opacity))',
            },
            '&:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '&.btn-primary:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '&.btn-primary:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-secondary:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '&.btn-secondary:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-accent:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '&.btn-accent:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '&.btn-primary .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '&.btn-secondary .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '&.btn-accent .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '&.btn-info .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--in) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--in) / var(--tw-text-opacity))',
            },
            '&.btn-success .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--su) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--su) / var(--tw-text-opacity))',
            },
            '&.btn-warning .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--wa) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wa) / var(--tw-text-opacity))',
            },
            '&.btn-error .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--er) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--er) / var(--tw-text-opacity))',
            },
            '&.btn-primary': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '&.btn-primary:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '&.btn-secondary': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '&.btn-secondary:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--sf, var(--s)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '&.btn-accent': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '&.btn-accent:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--af, var(--a)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '&.btn-success': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--su) / var(--tw-text-opacity))',
            },
            '&.btn-success:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--su) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--su) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--suc, var(--nc)) / var(--tw-text-opacity))',
            },
            '&.btn-info': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--in) / var(--tw-text-opacity))',
            },
            '&.btn-info:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--in) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--inc, var(--nc)) / var(--tw-text-opacity))',
            },
            '&.btn-warning': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wa) / var(--tw-text-opacity))',
            },
            '&.btn-warning:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--wa) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--wa) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wac, var(--nc)) / var(--tw-text-opacity))',
            },
            '&.btn-error': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--er) / var(--tw-text-opacity))',
            },
            '&.btn-error:hover': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--er) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--er) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--erc, var(--nc)) / var(--tw-text-opacity))',
            },
            '.btn-outline& .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '.btn-outline&:hover .badge': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '.btn-outline&:hover .badge.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '.drawer-toggle:focus-visible ~ .drawer-content .drawer-button&': {
              outline: '2px solid hsl(var(--a))',
            },
            '.btn-outline& .badge-outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '.btn-outline&': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '.btn-outline&:hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--af, var(--a)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '&.btn-active': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--af, var(--a)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
            },
          }}
        >
          Button
        </button>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            display: 'flex',
            width: '100%',
            flexDirection: 'column',
            alignItems: 'center',
            justifyContent: 'space-between',
            gap: '1rem',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--er) / var(--tw-bg-opacity))',
            padding: '1rem',
            borderRadius: 'var(--rounded-box, 1rem)',
            '& > :not([hidden]) ~ :not([hidden])': {
              '--tw-space-y-reverse': '0',
              marginTop: 'calc(0.5rem * calc(1 - var(--tw-space-y-reverse)))',
              marginBottom: 'calc(0.5rem * var(--tw-space-y-reverse))',
            },
            '& > :where(*)': {
              display: 'flex',
              alignItems: 'center',
              gap: '0.5rem',
            },
            '@media (min-width: 768px)': {
              flexDirection: 'row',
              '& > :not([hidden]) ~ :not([hidden])': {
                '--tw-space-y-reverse': '0',
                marginTop: 'calc(0px * calc(1 - var(--tw-space-y-reverse)))',
                marginBottom: 'calc(0px * var(--tw-space-y-reverse))',
              },
            },
            '--tw-text-opacity': '1',
            color: 'hsl(var(--erc, var(--nc)) / var(--tw-text-opacity))',
            '--tw-shadow':
              '0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)',
            '--tw-shadow-colored':
              '0 10px 15px -3px var(--tw-shadow-color), 0 4px 6px -4px var(--tw-shadow-color)',
            boxShadow:
              'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
          }}
        >
          <div>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              css={{
                stroke: 'currentColor',
                flexShrink: '0',
                height: '1.5rem',
                width: '1.5rem',
              }}
              fill="none"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <span>Error! Task failed successfully.</span>
          </div>
        </div>

        <div
          css={{
            display: 'flex',
            width: '100%',
            flexDirection: 'column',
            alignItems: 'center',
            justifyContent: 'space-between',
            gap: '1rem',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
            padding: '1rem',
            borderRadius: 'var(--rounded-box, 1rem)',
            '& > :not([hidden]) ~ :not([hidden])': {
              '--tw-space-y-reverse': '0',
              marginTop: 'calc(0.5rem * calc(1 - var(--tw-space-y-reverse)))',
              marginBottom: 'calc(0.5rem * var(--tw-space-y-reverse))',
            },
            '& > :where(*)': {
              display: 'flex',
              alignItems: 'center',
              gap: '0.5rem',
            },
            '@media (min-width: 768px)': {
              flexDirection: 'row',
              '& > :not([hidden]) ~ :not([hidden])': {
                '--tw-space-y-reverse': '0',
                marginTop: 'calc(0px * calc(1 - var(--tw-space-y-reverse)))',
                marginBottom: 'calc(0px * var(--tw-space-y-reverse))',
              },
            },
            '--tw-shadow':
              '0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)',
            '--tw-shadow-colored':
              '0 10px 15px -3px var(--tw-shadow-color), 0 4px 6px -4px var(--tw-shadow-color)',
            boxShadow:
              'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
          }}
        >
          <div>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              css={{
                stroke: 'hsl(var(--in))',
                flexShrink: '0',
                width: '1.5rem',
                height: '1.5rem',
              }}
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              ></path>
            </svg>
            <span>12 unread messages. Tap to see.</span>
          </div>
        </div>
      </div>

      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <label
          htmlFor="my-modal"
          css={{
            display: 'inline-flex',
            flexShrink: '0',
            cursor: 'pointer',
            userSelect: 'none',
            flexWrap: 'wrap',
            alignItems: 'center',
            justifyContent: 'center',
            borderColor: 'hsl(var(--n) / var(--tw-border-opacity))',
            textAlign: 'center',
            transitionProperty:
              'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
            transitionDuration: '200ms',
            transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
            borderRadius: 'var(--rounded-btn, 0.5rem)',
            height: '3rem',
            paddingLeft: '1rem',
            paddingRight: '1rem',
            fontSize: '0.875rem',
            lineHeight: '1em',
            minHeight: '3rem',
            fontWeight: '600',
            textTransform: 'var(--btn-text-case, uppercase)',
            textDecorationLine: 'none',
            borderWidth: 'var(--border-btn, 1px)',
            animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
            '--tw-border-opacity': '1',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
            '--tw-text-opacity': '1',
            color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            '&.loading': {
              pointerEvents: 'none',
            },
            '&.loading:hover': {
              pointerEvents: 'none',
            },
            '&.loading:before': {
              marginRight: '0.5rem',
              height: '1rem',
              width: '1rem',
              borderRadius: '9999px',
              borderWidth: '2px',
              animation: 'spin 2s linear infinite',
              content: '""',
              borderTopColor: 'transparent',
              borderLeftColor: 'transparent',
              borderBottomColor: 'currentColor',
              borderRightColor: 'currentColor',
            },
            ':active:hover': {
              animation: 'none',
              transform: 'scale(var(--btn-focus-scale, 0.95))',
            },
            ':active:focus': {
              animation: 'none',
              transform: 'scale(var(--btn-focus-scale, 0.95))',
            },
            ':hover': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
            },
            ':focus-visible': {
              outline: '2px solid hsl(var(--nf))',
              outlineOffset: '2px',
            },
            '&.glass:hover': {
              '--glass-opacity': '25%',
              '--glass-border-opacity': '15%',
            },
            '&.glass.btn-active': {
              '--glass-opacity': '25%',
              '--glass-border-opacity': '15%',
            },
            '&.glass:focus-visible': {
              outline: '2px solid 0 0 2px currentColor',
            },
            '&.loading.btn-square:before': {
              marginRight: '0px',
            },
            '&.loading.btn-circle:before': {
              marginRight: '0px',
            },
            '&.loading.btn-xl:before': {
              height: '1.25rem',
              width: '1.25rem',
            },
            '&.loading.btn-lg:before': {
              height: '1.25rem',
              width: '1.25rem',
            },
            '&.loading.btn-sm:before': {
              height: '0.75rem',
              width: '0.75rem',
            },
            '&.loading.btn-xs:before': {
              height: '0.75rem',
              width: '0.75rem',
            },
            '.btn-group > input[type="radio"]:checked&': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-group > input[type="radio"]:checked&:focus-visible': {
              outline: '2px solid hsl(var(--p))',
            },
            '.btn-group > &:not(:first-of-type)': {
              marginLeft: '-1px',
              borderTopLeftRadius: '0px',
              borderBottomLeftRadius: '0px',
            },
            '.btn-group > &:not(:last-of-type)': {
              borderTopRightRadius: '0px',
              borderBottomRightRadius: '0px',
            },
            '@media (prefers-reduced-motion: reduce)': {
              '&.loading:before': {
                animation: 'spin 10s linear infinite',
              },
            },
          }}
          className="modal-button"
        >
          open modal
        </label>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <label
          css={{
            position: 'relative',
            display: 'inline-grid',
            userSelect: 'none',
            placeContent: 'center',
            cursor: 'pointer',
            '& > *': {
              gridColumnStart: '1',
              gridRowStart: '1',
              transitionDuration: '300ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              transitionProperty: 'transform, opacity',
            },
            '& input': {
              appearance: 'none',
            },
            '& .swap-on': {
              opacity: '0',
            },
            '& .swap-indeterminate': {
              opacity: '0',
            },
            '& input:indeterminate ~ .swap-on': {
              opacity: '0',
            },
            '& input:checked ~ .swap-off': {
              opacity: '0',
            },
            '&.swap-active .swap-off': {
              opacity: '0',
            },
            '& input:indeterminate ~ .swap-off': {
              opacity: '0',
            },
            '& input:checked ~ .swap-on': {
              opacity: '1',
            },
            '& input:indeterminate ~ .swap-indeterminate': {
              opacity: '1',
            },
          }}
          className="swap"
        >
          <input type="checkbox" />
          <div
            css={{
              '.swap &': {
                opacity: '0',
              },
              '.swap input:indeterminate ~ &': {
                opacity: '0',
              },
              '.swap input:checked ~ &': {
                opacity: '1',
              },
              '.swap-active &': {
                opacity: '1',
              },
              '.swap-rotate &': {
                '--tw-rotate': '45deg',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.swap-rotate input:indeterminate ~ &': {
                '--tw-rotate': '45deg',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.swap-rotate input:checked ~ &': {
                '--tw-rotate': '0deg',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.swap-rotate.swap-active &': {
                '--tw-rotate': '0deg',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.swap-flip &': {
                transform: 'rotateY(180deg)',
                backfaceVisibility: 'hidden',
                opacity: '1',
              },
              '.swap-flip input:indeterminate ~ &': {
                transform: 'rotateY(180deg)',
                backfaceVisibility: 'hidden',
                opacity: '1',
              },
              '.swap-flip input:checked ~ &': {
                transform: 'rotateY(0deg)',
              },
              '.swap-flip.swap-active &': {
                transform: 'rotateY(0deg)',
              },
            }}
          >
            ON
          </div>
          <div
            css={{
              '.swap input:checked ~ &': {
                opacity: '0',
              },
              '.swap.swap-active &': {
                opacity: '0',
              },
              '.swap input:indeterminate ~ &': {
                opacity: '0',
              },
              '.swap-rotate input:checked ~ &': {
                '--tw-rotate': '-45deg',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.swap-rotate.swap-active &': {
                '--tw-rotate': '-45deg',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.swap-rotate input:indeterminate ~ &': {
                '--tw-rotate': '-45deg',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.swap-flip input:checked ~ &': {
                transform: 'rotateY(-180deg)',
                backfaceVisibility: 'hidden',
                opacity: '1',
              },
              '.swap-flip.swap-active &': {
                transform: 'rotateY(-180deg)',
                backfaceVisibility: 'hidden',
                opacity: '1',
              },
              '.swap-flip input:indeterminate ~ &': {
                transform: 'rotateY(-180deg)',
                backfaceVisibility: 'hidden',
                opacity: '1',
              },
            }}
          >
            OFF
          </div>
        </label>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            display: 'inline-flex',
            alignItems: 'center',
            justifyContent: 'center',
            transitionProperty:
              'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
            transitionDuration: '200ms',
            transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
            height: '1.25rem',
            fontSize: '0.875rem',
            lineHeight: '1.25rem',
            width: 'fit-content',
            paddingLeft: '0.563rem',
            paddingRight: '0.563rem',
            borderWidth: '1px',
            '--tw-border-opacity': '1',
            borderColor: 'transparent',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
            '--tw-text-opacity': '1',
            color: 'hsl(var(--inc, var(--nc)) / var(--tw-text-opacity))',
            borderRadius: 'var(--rounded-badge, 1.9rem)',
            '.btn-outline &': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-primary &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-secondary &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-accent &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '.btn-outline &.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
            },
            '.btn-outline:hover &': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--bc) / var(--tw-text-opacity))',
            },
            '.btn-outline:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-primary:hover &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-primary:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-secondary:hover &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-secondary:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-accent:hover &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-accent:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '.badge-outline&': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--in) / var(--tw-text-opacity))',
            },
            gap: '0.5rem',
          }}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            css={{
              display: 'inline-block',
              width: '1rem',
              height: '1rem',
              stroke: 'currentColor',
            }}
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M6 18L18 6M6 6l12 12"
            ></path>
          </svg>
          info
        </div>
        <div
          css={{
            display: 'inline-flex',
            alignItems: 'center',
            justifyContent: 'center',
            transitionProperty:
              'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
            transitionDuration: '200ms',
            transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
            height: '1.25rem',
            fontSize: '0.875rem',
            lineHeight: '1.25rem',
            width: 'fit-content',
            paddingLeft: '0.563rem',
            paddingRight: '0.563rem',
            borderWidth: '1px',
            '--tw-border-opacity': '1',
            borderColor: 'transparent',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--su) / var(--tw-bg-opacity))',
            '--tw-text-opacity': '1',
            color: 'hsl(var(--suc, var(--nc)) / var(--tw-text-opacity))',
            borderRadius: 'var(--rounded-badge, 1.9rem)',
            '.btn-outline &': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-primary &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-secondary &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-accent &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '.btn-outline &.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
            },
            '.btn-outline:hover &': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--bc) / var(--tw-text-opacity))',
            },
            '.btn-outline:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-primary:hover &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-primary:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-secondary:hover &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-secondary:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-accent:hover &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-accent:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '.badge-outline&': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--su) / var(--tw-text-opacity))',
            },
            gap: '0.5rem',
          }}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            css={{
              display: 'inline-block',
              width: '1rem',
              height: '1rem',
              stroke: 'currentColor',
            }}
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M6 18L18 6M6 6l12 12"
            ></path>
          </svg>
          success
        </div>
        <div
          css={{
            display: 'inline-flex',
            alignItems: 'center',
            justifyContent: 'center',
            transitionProperty:
              'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
            transitionDuration: '200ms',
            transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
            height: '1.25rem',
            fontSize: '0.875rem',
            lineHeight: '1.25rem',
            width: 'fit-content',
            paddingLeft: '0.563rem',
            paddingRight: '0.563rem',
            borderWidth: '1px',
            '--tw-border-opacity': '1',
            borderColor: 'transparent',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--wa) / var(--tw-bg-opacity))',
            '--tw-text-opacity': '1',
            color: 'hsl(var(--wac, var(--nc)) / var(--tw-text-opacity))',
            borderRadius: 'var(--rounded-badge, 1.9rem)',
            '.btn-outline &': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-primary &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-secondary &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-accent &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '.btn-outline &.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
            },
            '.btn-outline:hover &': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--bc) / var(--tw-text-opacity))',
            },
            '.btn-outline:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-primary:hover &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-primary:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-secondary:hover &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-secondary:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-accent:hover &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-accent:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '.badge-outline&': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wa) / var(--tw-text-opacity))',
            },
            gap: '0.5rem',
          }}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            css={{
              display: 'inline-block',
              width: '1rem',
              height: '1rem',
              stroke: 'currentColor',
            }}
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M6 18L18 6M6 6l12 12"
            ></path>
          </svg>
          warning
        </div>
        <div
          css={{
            display: 'inline-flex',
            alignItems: 'center',
            justifyContent: 'center',
            transitionProperty:
              'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
            transitionDuration: '200ms',
            transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
            height: '1.25rem',
            fontSize: '0.875rem',
            lineHeight: '1.25rem',
            width: 'fit-content',
            paddingLeft: '0.563rem',
            paddingRight: '0.563rem',
            borderWidth: '1px',
            '--tw-border-opacity': '1',
            borderColor: 'transparent',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--er) / var(--tw-bg-opacity))',
            '--tw-text-opacity': '1',
            color: 'hsl(var(--erc, var(--nc)) / var(--tw-text-opacity))',
            borderRadius: 'var(--rounded-badge, 1.9rem)',
            '.btn-outline &': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-primary &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-secondary &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-accent &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '.btn-outline &.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
              backgroundColor: 'transparent',
            },
            '.btn-outline:hover &': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--bc) / var(--tw-text-opacity))',
            },
            '.btn-outline:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-primary:hover &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--p) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-primary:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-secondary:hover &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--s) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-secondary:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-accent:hover &': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--a) / var(--tw-text-opacity))',
            },
            '.btn-outline.btn-accent:hover &.outline': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '.badge-outline&': {
              '--tw-text-opacity': '1',
              color: 'hsl(var(--er) / var(--tw-text-opacity))',
            },
            gap: '0.5rem',
          }}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            css={{
              display: 'inline-block',
              width: '1rem',
              height: '1rem',
              stroke: 'currentColor',
            }}
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M6 18L18 6M6 6l12 12"
            ></path>
          </svg>
          error
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <span
          css={{
            '*:root &': {
              lineHeight: '1em',
            },
            display: 'inline-flex',
            '& > *': {
              height: '1em',
              display: 'inline-block',
              overflowY: 'hidden',
            },
            '& > *:before': {
              position: 'relative',
              content:
                '"00\\A 01\\A 02\\A 03\\A 04\\A 05\\A 06\\A 07\\A 08\\A 09\\A 10\\A 11\\A 12\\A 13\\A 14\\A 15\\A 16\\A 17\\A 18\\A 19\\A 20\\A 21\\A 22\\A 23\\A 24\\A 25\\A 26\\A 27\\A 28\\A 29\\A 30\\A 31\\A 32\\A 33\\A 34\\A 35\\A 36\\A 37\\A 38\\A 39\\A 40\\A 41\\A 42\\A 43\\A 44\\A 45\\A 46\\A 47\\A 48\\A 49\\A 50\\A 51\\A 52\\A 53\\A 54\\A 55\\A 56\\A 57\\A 58\\A 59\\A 60\\A 61\\A 62\\A 63\\A 64\\A 65\\A 66\\A 67\\A 68\\A 69\\A 70\\A 71\\A 72\\A 73\\A 74\\A 75\\A 76\\A 77\\A 78\\A 79\\A 80\\A 81\\A 82\\A 83\\A 84\\A 85\\A 86\\A 87\\A 88\\A 89\\A 90\\A 91\\A 92\\A 93\\A 94\\A 95\\A 96\\A 97\\A 98\\A 99\\A"',
              whiteSpace: 'pre',
              top: 'calc(var(--value) * -1em)',
              textAlign: 'center',
              transition: 'all 1s cubic-bezier(1, 0, 0, 1)',
            },
          }}
        >
          <span
            css={{
              '--value': '25',
            }}
          ></span>
        </span>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            display: 'inline-grid',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--b1) / var(--tw-bg-opacity))',
            '--tw-text-opacity': '1',
            color: 'hsl(var(--bc) / var(--tw-text-opacity))',
            borderRadius: 'var(--rounded-box, 1rem)',
            gridAutoFlow: 'column',
            overflowX: 'auto',
            ' > :not([hidden]) ~ :not([hidden])': {
              '--tw-divide-x-reverse': '0',
              borderRightWidth: 'calc(1px * var(--tw-divide-x-reverse))',
              borderLeftWidth:
                'calc(1px * calc(1 - var(--tw-divide-x-reverse)))',
              '--tw-divide-y-reverse': '0',
              borderTopWidth:
                'calc(0px * calc(1 - var(--tw-divide-y-reverse)))',
              borderBottomWidth: 'calc(0px * var(--tw-divide-y-reverse))',
            },
            '--tw-shadow':
              '0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)',
            '--tw-shadow-colored':
              '0 1px 3px 0 var(--tw-shadow-color), 0 1px 2px -1px var(--tw-shadow-color)',
            boxShadow:
              'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
          }}
        >
          <div
            css={{
              display: 'inline-grid',
              width: '100%',
              gridTemplateColumns: 'repeat(1, 1fr)',
              columnGap: '1rem',
              borderColor: 'hsl(var(--bc) / var(--tw-border-opacity))',
              '--tw-border-opacity': '0.1',
              paddingLeft: '1.5rem',
              paddingRight: '1.5rem',
              paddingTop: '1rem',
              paddingBottom: '1rem',
            }}
          >
            <div
              css={{
                gridColumnStart: '2',
                gridRow: 'span 3 / span 3',
                gridRowStart: '1',
                placeSelf: 'center',
                justifySelf: 'end',
                color: 'hsl(var(--p))',
              }}
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                css={{
                  display: 'inline-block',
                  width: '2rem',
                  height: '2rem',
                  stroke: 'currentColor',
                }}
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                ></path>
              </svg>
            </div>
            <div
              css={{
                gridColumnStart: '1',
                whiteSpace: 'nowrap',
                opacity: '0.6',
              }}
            >
              Total Likes
            </div>
            <div
              css={{
                gridColumnStart: '1',
                whiteSpace: 'nowrap',
                fontSize: '2.25rem',
                lineHeight: '2.5rem',
                fontWeight: '800',
                color: 'hsl(var(--p))',
              }}
            >
              25.6K
            </div>
            <div
              css={{
                gridColumnStart: '1',
                whiteSpace: 'nowrap',
                fontSize: '0.75rem',
                lineHeight: '1rem',
                opacity: '0.6',
              }}
            >
              21% more than last month
            </div>
          </div>

          <div
            css={{
              display: 'inline-grid',
              width: '100%',
              gridTemplateColumns: 'repeat(1, 1fr)',
              columnGap: '1rem',
              borderColor: 'hsl(var(--bc) / var(--tw-border-opacity))',
              '--tw-border-opacity': '0.1',
              paddingLeft: '1.5rem',
              paddingRight: '1.5rem',
              paddingTop: '1rem',
              paddingBottom: '1rem',
            }}
          >
            <div
              css={{
                gridColumnStart: '2',
                gridRow: 'span 3 / span 3',
                gridRowStart: '1',
                placeSelf: 'center',
                justifySelf: 'end',
                color: 'hsl(var(--s))',
              }}
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                css={{
                  display: 'inline-block',
                  width: '2rem',
                  height: '2rem',
                  stroke: 'currentColor',
                }}
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  d="M13 10V3L4 14h7v7l9-11h-7z"
                ></path>
              </svg>
            </div>
            <div
              css={{
                gridColumnStart: '1',
                whiteSpace: 'nowrap',
                opacity: '0.6',
              }}
            >
              Page Views
            </div>
            <div
              css={{
                gridColumnStart: '1',
                whiteSpace: 'nowrap',
                fontSize: '2.25rem',
                lineHeight: '2.5rem',
                fontWeight: '800',
                color: 'hsl(var(--s))',
              }}
            >
              2.6M
            </div>
            <div
              css={{
                gridColumnStart: '1',
                whiteSpace: 'nowrap',
                fontSize: '0.75rem',
                lineHeight: '1rem',
                opacity: '0.6',
              }}
            >
              21% more than last month
            </div>
          </div>

          <div
            css={{
              display: 'inline-grid',
              width: '100%',
              gridTemplateColumns: 'repeat(1, 1fr)',
              columnGap: '1rem',
              borderColor: 'hsl(var(--bc) / var(--tw-border-opacity))',
              '--tw-border-opacity': '0.1',
              paddingLeft: '1.5rem',
              paddingRight: '1.5rem',
              paddingTop: '1rem',
              paddingBottom: '1rem',
            }}
          >
            <div
              css={{
                gridColumnStart: '2',
                gridRow: 'span 3 / span 3',
                gridRowStart: '1',
                placeSelf: 'center',
                justifySelf: 'end',
                color: 'hsl(var(--s))',
              }}
            >
              <div
                css={{
                  position: 'relative',
                  display: 'inline-flex',
                  '& > div': {
                    display: 'block',
                    aspectRatio: '1 / 1',
                    overflow: 'hidden',
                  },
                  '& img': {
                    height: '100%',
                    width: '100%',
                    objectFit: 'cover',
                  },
                  '&.placeholder > div': {
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                  },
                  '.avatar-group &': {
                    overflow: 'hidden',
                    borderRadius: '9999px',
                    borderWidth: '4px',
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--b1) / var(--tw-border-opacity))',
                  },
                  '&.online:before': {
                    content: '""',
                    position: 'absolute',
                    zIndex: '10',
                    display: 'block',
                    borderRadius: '9999px',
                    '--tw-bg-opacity': '1',
                    backgroundColor: 'hsl(var(--su) / var(--tw-bg-opacity))',
                    width: '15%',
                    height: '15%',
                    top: '7%',
                    right: '7%',
                    boxShadow: '0 0 0 2px hsl(var(--b1))',
                  },
                  '&.offline:before': {
                    content: '""',
                    position: 'absolute',
                    zIndex: '10',
                    display: 'block',
                    borderRadius: '9999px',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--b3, var(--b2)) / var(--tw-bg-opacity))',
                    width: '15%',
                    height: '15%',
                    top: '7%',
                    right: '7%',
                    boxShadow: '0 0 0 2px hsl(var(--b1))',
                  },
                }}
                className="online"
              >
                <div
                  css={{
                    width: '4rem',
                    borderRadius: '9999px',
                  }}
                >
                  <img src="https://api.lorem.space/image/face?w=128&h=128" />
                </div>
              </div>
            </div>
            <div
              css={{
                gridColumnStart: '1',
                whiteSpace: 'nowrap',
                fontSize: '2.25rem',
                lineHeight: '2.5rem',
                fontWeight: '800',
              }}
            >
              86%
            </div>
            <div
              css={{
                gridColumnStart: '1',
                whiteSpace: 'nowrap',
                opacity: '0.6',
              }}
            >
              Tasks done
            </div>
            <div
              css={{
                gridColumnStart: '1',
                whiteSpace: 'nowrap',
                fontSize: '0.75rem',
                lineHeight: '1rem',
                opacity: '0.6',
                color: 'hsl(var(--s))',
              }}
            >
              31 tasks remaining
            </div>
          </div>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            position: 'relative',
            display: 'flex',
            flexDirection: 'row',
            overflow: 'hidden',
            borderRadius: 'var(--rounded-box, 1rem)',
            ':focus': {
              outline: '2px solid transparent',
              outlineOffset: '2px',
            },
            '& figure': {
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
            },
            '&.image-full': {
              display: 'grid',
            },
            '&.image-full:before': {
              position: 'relative',
              content: '""',
              zIndex: '10',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              opacity: '0.75',
              borderRadius: 'var(--rounded-box, 1rem)',
              gridColumnStart: '1',
              gridRowStart: '1',
            },
            '&.image-full > *': {
              gridColumnStart: '1',
              gridRowStart: '1',
            },
            '&.image-full > figure img': {
              height: '100%',
              objectFit: 'cover',
            },
            '&.image-full > .card-body': {
              position: 'relative',
              zIndex: '20',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            ':focus-visible': {
              outline: '2px solid currentColor',
              outlineOffset: '2px',
            },
            '&.bordered': {
              borderWidth: '1px',
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
            },
            '&.compact .card-body': {
              padding: '1rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
            },
            alignItems: 'stretch',
            '& figure > *': {
              maxWidth: 'unset',
            },
            ':where(& figure > *)': {
              width: '100%',
              height: '100%',
              objectFit: 'cover',
            },
            backgroundColor: 'hsl(var(--b1))',
            '--tw-shadow':
              '0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)',
            '--tw-shadow-colored':
              '0 20px 25px -5px var(--tw-shadow-color), 0 8px 10px -6px var(--tw-shadow-color)',
            boxShadow:
              'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
          }}
        >
          <figure>
            <img
              src="https://api.lorem.space/image/movie?w=200&h=280"
              alt="Movie"
            />
          </figure>
          <div
            css={{
              '.card.image-full > &': {
                position: 'relative',
                zIndex: '20',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.card.compact &': {
                padding: '1rem',
                fontSize: '0.875rem',
                lineHeight: '1.25rem',
              },
              display: 'flex',
              flex: '1 1 auto',
              flexDirection: 'column',
              padding: 'var(--padding-card, 2rem)',
              gap: '0.5rem',
              '& :where(p)': {
                flexGrow: '1',
              },
              '.card-compact &': {
                padding: '1rem',
                fontSize: '0.875rem',
                lineHeight: '1.25rem',
              },
              '.card-normal &': {
                padding: 'var(--padding-card, 2rem)',
                fontSize: '1rem',
                lineHeight: '1.5rem',
              },
            }}
          >
            <h2
              css={{
                display: 'flex',
                alignItems: 'center',
                gap: '0.5rem',
                fontSize: '1.25rem',
                lineHeight: '1.75rem',
                fontWeight: '600',
                '.card-compact &': {
                  marginBottom: '0.25rem',
                },
                '.card-normal &': {
                  marginBottom: '0.75rem',
                },
              }}
            >
              New movie is released!
            </h2>
            <p>Click the button to watch on Jetflix app.</p>
            <div
              css={{
                display: 'flex',
                flexWrap: 'wrap',
                alignItems: 'flex-start',
                gap: '0.5rem',
                justifyContent: 'flex-end',
              }}
            >
              <button
                css={{
                  display: 'inline-flex',
                  flexShrink: '0',
                  cursor: 'pointer',
                  userSelect: 'none',
                  flexWrap: 'wrap',
                  alignItems: 'center',
                  justifyContent: 'center',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                  textAlign: 'center',
                  transitionProperty:
                    'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
                  transitionDuration: '200ms',
                  transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
                  borderRadius: 'var(--rounded-btn, 0.5rem)',
                  height: '3rem',
                  paddingLeft: '1rem',
                  paddingRight: '1rem',
                  fontSize: '0.875rem',
                  lineHeight: '1em',
                  minHeight: '3rem',
                  fontWeight: '600',
                  textTransform: 'var(--btn-text-case, uppercase)',
                  textDecorationLine: 'none',
                  borderWidth: 'var(--border-btn, 1px)',
                  animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
                  '--tw-border-opacity': '1',
                  '--tw-bg-opacity': '1',
                  backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                  '--tw-text-opacity': '1',
                  color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  '&.loading': {
                    pointerEvents: 'none',
                  },
                  '&.loading:hover': {
                    pointerEvents: 'none',
                  },
                  '&.loading:before': {
                    marginRight: '0.5rem',
                    height: '1rem',
                    width: '1rem',
                    borderRadius: '9999px',
                    borderWidth: '2px',
                    animation: 'spin 2s linear infinite',
                    content: '""',
                    borderTopColor: 'transparent',
                    borderLeftColor: 'transparent',
                    borderBottomColor: 'currentColor',
                    borderRightColor: 'currentColor',
                  },
                  ':active:hover': {
                    animation: 'none',
                    transform: 'scale(var(--btn-focus-scale, 0.95))',
                  },
                  ':active:focus': {
                    animation: 'none',
                    transform: 'scale(var(--btn-focus-scale, 0.95))',
                  },
                  ':hover': {
                    '--tw-border-opacity': '1',
                    borderColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                  },
                  ':focus-visible': {
                    outline: '2px solid hsl(var(--p))',
                    outlineOffset: '2px',
                  },
                  '&.glass:hover': {
                    '--glass-opacity': '25%',
                    '--glass-border-opacity': '15%',
                  },
                  '&.glass.btn-active': {
                    '--glass-opacity': '25%',
                    '--glass-border-opacity': '15%',
                  },
                  '&.glass:focus-visible': {
                    outline: '2px solid 0 0 2px currentColor',
                  },
                  '&.loading.btn-square:before': {
                    marginRight: '0px',
                  },
                  '&.loading.btn-circle:before': {
                    marginRight: '0px',
                  },
                  '&.loading.btn-xl:before': {
                    height: '1.25rem',
                    width: '1.25rem',
                  },
                  '&.loading.btn-lg:before': {
                    height: '1.25rem',
                    width: '1.25rem',
                  },
                  '&.loading.btn-sm:before': {
                    height: '0.75rem',
                    width: '0.75rem',
                  },
                  '&.loading.btn-xs:before': {
                    height: '0.75rem',
                    width: '0.75rem',
                  },
                  '.btn-group > input[type="radio"]:checked&': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                  '.btn-group > input[type="radio"]:checked&:focus-visible': {
                    outline: '2px solid hsl(var(--p))',
                  },
                  '.btn-group > &:not(:first-of-type)': {
                    marginLeft: '-1px',
                    borderTopLeftRadius: '0px',
                    borderBottomLeftRadius: '0px',
                  },
                  '.btn-group > &:not(:last-of-type)': {
                    borderTopRightRadius: '0px',
                    borderBottomRightRadius: '0px',
                  },
                  '@media (prefers-reduced-motion: reduce)': {
                    '&.loading:before': {
                      animation: 'spin 10s linear infinite',
                    },
                  },
                  '.btn-outline& .badge': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                  '.btn-outline&:hover .badge': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--p) / var(--tw-text-opacity))',
                  },
                  '.btn-outline&:hover .badge.outline': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                  '.drawer-toggle:focus-visible ~ .drawer-content .drawer-button&':
                    {
                      outline: '2px solid hsl(var(--p))',
                    },
                  '.btn-outline& .badge-outline': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                    backgroundColor: 'transparent',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--p) / var(--tw-text-opacity))',
                  },
                  '.btn-outline&': {
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--p) / var(--tw-text-opacity))',
                  },
                  '.btn-outline&:hover': {
                    '--tw-border-opacity': '1',
                    borderColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                  '&.btn-active': {
                    '--tw-border-opacity': '1',
                    borderColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                  },
                }}
              >
                Watch
              </button>
            </div>
          </div>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          className="tooltip"
          css={{
            position: 'relative',
            display: 'inline-block',
            '--tooltip-offset': 'calc(100% + 1px + var(--tooltip-tail, 0px))',
            textAlign: 'center',
            '--tooltip-tail': '3px',
            '--tooltip-color': 'hsl(var(--n))',
            '--tooltip-text-color': 'hsl(var(--nc))',
            '--tooltip-tail-offset': 'calc(100% + 1px - var(--tooltip-tail))',
            ':before': {
              position: 'absolute',
              pointerEvents: 'none',
              content: 'attr(data-tip)',
              transform: 'translateX(-50%)',
              top: 'auto',
              left: '50%',
              right: 'auto',
              bottom: 'var(--tooltip-offset)',
              maxWidth: '20rem',
              borderRadius: '0.25rem',
              paddingLeft: '0.5rem',
              paddingRight: '0.5rem',
              paddingTop: '0.25rem',
              paddingBottom: '0.25rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
              backgroundColor: 'var(--tooltip-color)',
              color: 'var(--tooltip-text-color)',
              width: 'max-content',
              opacity: '0',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDelay: '100ms',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
            },
            ':after': {
              opacity: '0',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDelay: '100ms',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              position: 'absolute',
              content: '""',
              borderStyle: 'solid',
              borderWidth: 'var(--tooltip-tail, 0)',
              width: '0',
              height: '0',
              display: 'block',
              transform: 'translateX(-50%)',
              borderColor:
                'var(--tooltip-color) transparent transparent transparent',
              top: 'auto',
              left: '50%',
              right: 'auto',
              bottom: 'var(--tooltip-tail-offset)',
            },
            '&.tooltip-open:before': {
              opacity: '1',
              transitionDelay: '75ms',
            },
            '&.tooltip-open:after': {
              opacity: '1',
              transitionDelay: '75ms',
            },
            ':hover:before': {
              opacity: '1',
              transitionDelay: '75ms',
            },
            ':hover:after': {
              opacity: '1',
              transitionDelay: '75ms',
            },
          }}
          data-tip="hello"
        >
          <button
            css={{
              display: 'inline-flex',
              flexShrink: '0',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              borderColor: 'hsl(var(--n) / var(--tw-border-opacity))',
              textAlign: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              borderRadius: 'var(--rounded-btn, 0.5rem)',
              height: '3rem',
              paddingLeft: '1rem',
              paddingRight: '1rem',
              fontSize: '0.875rem',
              lineHeight: '1em',
              minHeight: '3rem',
              fontWeight: '600',
              textTransform: 'var(--btn-text-case, uppercase)',
              textDecorationLine: 'none',
              borderWidth: 'var(--border-btn, 1px)',
              animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              '&.loading': {
                pointerEvents: 'none',
              },
              '&.loading:hover': {
                pointerEvents: 'none',
              },
              '&.loading:before': {
                marginRight: '0.5rem',
                height: '1rem',
                width: '1rem',
                borderRadius: '9999px',
                borderWidth: '2px',
                animation: 'spin 2s linear infinite',
                content: '""',
                borderTopColor: 'transparent',
                borderLeftColor: 'transparent',
                borderBottomColor: 'currentColor',
                borderRightColor: 'currentColor',
              },
              ':active:hover': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':active:focus': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':hover': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
              },
              ':focus-visible': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '&.glass:hover': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass.btn-active': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass:focus-visible': {
                outline: '2px solid 0 0 2px currentColor',
              },
              '&.loading.btn-square:before': {
                marginRight: '0px',
              },
              '&.loading.btn-circle:before': {
                marginRight: '0px',
              },
              '&.loading.btn-xl:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-lg:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-sm:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '&.loading.btn-xs:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-group > input[type="radio"]:checked&': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-group > input[type="radio"]:checked&:focus-visible': {
                outline: '2px solid hsl(var(--p))',
              },
              '.btn-group > &:not(:first-of-type)': {
                marginLeft: '-1px',
                borderTopLeftRadius: '0px',
                borderBottomLeftRadius: '0px',
              },
              '.btn-group > &:not(:last-of-type)': {
                borderTopRightRadius: '0px',
                borderBottomRightRadius: '0px',
              },
              '@media (prefers-reduced-motion: reduce)': {
                '&.loading:before': {
                  animation: 'spin 10s linear infinite',
                },
              },
            }}
          >
            Bottom
          </button>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            display: 'flex',
            flexDirection: 'column',
          }}
        >
          <label
            css={{
              display: 'flex',
              userSelect: 'none',
              alignItems: 'center',
              justifyContent: 'space-between',
              paddingLeft: '0.25rem',
              paddingRight: '0.25rem',
              paddingTop: '0.5rem',
              paddingBottom: '0.5rem',
              '& a:hover': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              cursor: 'pointer',
            }}
          >
            <span
              css={{
                fontSize: '0.875rem',
                lineHeight: '1.25rem',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              }}
            >
              Remember me
            </span>
            <input
              type="checkbox"
              css={{
                flexShrink: '0',
                '--chkbg': 'var(--p)',
                '--chkfg': 'var(--pc)',
                height: '1.5rem',
                width: '1.5rem',
                cursor: 'pointer',
                appearance: 'none',
                borderWidth: '1px',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-border-opacity': '1',
                borderRadius: 'var(--rounded-btn, 0.5rem)',
                ':focus-visible': {
                  outline: '2px solid hsl(var(--p))',
                  outlineOffset: '2px',
                },
                ':checked': {
                  '--tw-bg-opacity': '1',
                  backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                  backgroundRepeat: 'no-repeat',
                  animation:
                    'checkmark var(--animation-input, 0.2s) ease-in-out',
                  backgroundImage:
                    'linear-gradient(-45deg, transparent 65%, hsl(var(--chkbg)) 65.99%), linear-gradient(45deg, transparent 75%, hsl(var(--chkbg)) 75.99%), linear-gradient(-45deg, hsl(var(--chkbg)) 40%, transparent 40.99%), linear-gradient(45deg, hsl(var(--chkbg)) 30%, hsl(var(--chkfg)) 30.99%, hsl(var(--chkfg)) 40%, transparent 40.99%), linear-gradient(-45deg, hsl(var(--chkfg)) 50%, hsl(var(--chkbg)) 50.99%)',
                  '--tw-border-opacity': '1',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                  '--tw-text-opacity': '1',
                  color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                },
                ':indeterminate': {
                  '--tw-bg-opacity': '1',
                  backgroundColor: 'hsl(var(--bc) / var(--tw-bg-opacity))',
                  backgroundRepeat: 'no-repeat',
                  animation:
                    'checkmark var(--animation-input, 0.2s) ease-in-out',
                  backgroundImage:
                    'linear-gradient(90deg, transparent 80%, hsl(var(--chkbg)) 80%), linear-gradient(-90deg, transparent 80%, hsl(var(--chkbg)) 80%), linear-gradient(0deg, hsl(var(--chkbg)) 43%, hsl(var(--chkfg)) 43%, hsl(var(--chkfg)) 57%, hsl(var(--chkbg)) 57%)',
                },
                ':disabled': {
                  cursor: 'not-allowed',
                  borderColor: 'transparent',
                  '--tw-bg-opacity': '1',
                  backgroundColor: 'hsl(var(--bc) / var(--tw-bg-opacity))',
                  opacity: '0.2',
                },
                'body[dir="rtl"] &': {
                  '--chkbg': 'var(--bc)',
                  '--chkfg': 'var(--b1)',
                },
                'body[dir="rtl"] &:checked': {
                  backgroundImage:
                    'linear-gradient(45deg, transparent 65%, hsl(var(--chkbg)) 65.99%), linear-gradient(-45deg, transparent 75%, hsl(var(--chkbg)) 75.99%), linear-gradient(45deg, hsl(var(--chkbg)) 40%, transparent 40.99%), linear-gradient(-45deg, hsl(var(--chkbg)) 30%, hsl(var(--chkfg)) 30.99%, hsl(var(--chkfg)) 40%, transparent 40.99%), linear-gradient(45deg, hsl(var(--chkfg)) 50%, hsl(var(--chkbg)) 50.99%)',
                },
                ':hover': {
                  '--tw-border-opacity': '1',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                },
              }}
            />
          </label>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            display: 'flex',
            flexDirection: 'column',
            width: '100%',
            maxWidth: '20rem',
          }}
        >
          <label
            css={{
              display: 'flex',
              userSelect: 'none',
              alignItems: 'center',
              justifyContent: 'space-between',
              paddingLeft: '0.25rem',
              paddingRight: '0.25rem',
              paddingTop: '0.5rem',
              paddingBottom: '0.5rem',
              '& a:hover': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
            }}
          >
            <span
              css={{
                fontSize: '0.875rem',
                lineHeight: '1.25rem',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              }}
            >
              What is your name?
            </span>
            <span
              css={{
                fontSize: '0.75rem',
                lineHeight: '1rem',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              }}
            >
              Alt label
            </span>
          </label>
          <input
            type="text"
            placeholder="Type here"
            css={{
              flexShrink: '1',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              height: '3rem',
              paddingLeft: '1rem',
              paddingRight: '1rem',
              fontSize: '0.875rem',
              lineHeight: '2',
              borderWidth: '1px',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--b1) / var(--tw-bg-opacity))',
              borderRadius: 'var(--rounded-btn, 0.5rem)',
              '.input-group > &': {
                borderRadius: '0px',
              },
              ':focus': {
                outline: '2px solid hsl(var(--s))',
                outlineOffset: '2px',
              },
              width: '100%',
              maxWidth: '20rem',
            }}
          />
          <label
            css={{
              display: 'flex',
              userSelect: 'none',
              alignItems: 'center',
              justifyContent: 'space-between',
              paddingLeft: '0.25rem',
              paddingRight: '0.25rem',
              paddingTop: '0.5rem',
              paddingBottom: '0.5rem',
              '& a:hover': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
            }}
          >
            <span
              css={{
                fontSize: '0.75rem',
                lineHeight: '1rem',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              }}
            >
              Alt label
            </span>
            <span
              css={{
                fontSize: '0.75rem',
                lineHeight: '1rem',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              }}
            >
              Alt label
            </span>
          </label>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            display: 'flex',
            flexDirection: 'column',
          }}
        >
          <label
            css={{
              display: 'flex',
              userSelect: 'none',
              alignItems: 'center',
              justifyContent: 'space-between',
              paddingLeft: '0.25rem',
              paddingRight: '0.25rem',
              paddingTop: '0.5rem',
              paddingBottom: '0.5rem',
              '& a:hover': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              cursor: 'pointer',
            }}
          >
            <span
              css={{
                fontSize: '0.875rem',
                lineHeight: '1.25rem',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              }}
            >
              Red pill
            </span>
            <input
              type="radio"
              name="radio-6"
              css={{
                flexShrink: '0',
                '--chkbg': 'var(--bc)',
                height: '1.5rem',
                width: '1.5rem',
                cursor: 'pointer',
                appearance: 'none',
                borderRadius: '9999px',
                borderWidth: '1px',
                borderColor: 'hsl(var(--bc) / var(--tw-border-opacity))',
                '--tw-border-opacity': '0.2',
                transition:
                  'background, box-shadow var(--animation-input, 0.2s) ease-in-out',
                ':focus-visible': {
                  outline: '2px solid hsl(var(--bc))',
                  outlineOffset: '2px',
                },
                ':checked': {
                  '--tw-bg-opacity': '1',
                  backgroundColor: 'rgb(239 68 68 / var(--tw-bg-opacity))',
                  animation:
                    'radiomark var(--animation-input, 0.2s) ease-in-out',
                  boxShadow:
                    '0 0 0 4px hsl(var(--b1)) inset, 0 0 0 4px hsl(var(--b1)) inset',
                },
                ':disabled': {
                  cursor: 'not-allowed',
                  opacity: '0.2',
                },
              }} // checked
            />
          </label>
        </div>
        <div
          css={{
            display: 'flex',
            flexDirection: 'column',
          }}
        >
          <label
            css={{
              display: 'flex',
              userSelect: 'none',
              alignItems: 'center',
              justifyContent: 'space-between',
              paddingLeft: '0.25rem',
              paddingRight: '0.25rem',
              paddingTop: '0.5rem',
              paddingBottom: '0.5rem',
              '& a:hover': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              cursor: 'pointer',
            }}
          >
            <span
              css={{
                fontSize: '0.875rem',
                lineHeight: '1.25rem',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              }}
            >
              blue pill
            </span>
            <input
              type="radio"
              name="radio-6"
              css={{
                flexShrink: '0',
                '--chkbg': 'var(--bc)',
                height: '1.5rem',
                width: '1.5rem',
                cursor: 'pointer',
                appearance: 'none',
                borderRadius: '9999px',
                borderWidth: '1px',
                borderColor: 'hsl(var(--bc) / var(--tw-border-opacity))',
                '--tw-border-opacity': '0.2',
                transition:
                  'background, box-shadow var(--animation-input, 0.2s) ease-in-out',
                ':focus-visible': {
                  outline: '2px solid hsl(var(--bc))',
                  outlineOffset: '2px',
                },
                ':checked': {
                  '--tw-bg-opacity': '1',
                  backgroundColor: 'rgb(59 130 246 / var(--tw-bg-opacity))',
                  animation:
                    'radiomark var(--animation-input, 0.2s) ease-in-out',
                  boxShadow:
                    '0 0 0 4px hsl(var(--b1)) inset, 0 0 0 4px hsl(var(--b1)) inset',
                },
                ':disabled': {
                  cursor: 'not-allowed',
                  opacity: '0.2',
                },
              }} // checked
            />
          </label>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <input
          type="range"
          min="0"
          max="100"
          css={{
            height: '1.5rem',
            width: '100%',
            cursor: 'pointer',
            WebkitAppearance: 'none',
            '--range-shdw': 'var(--s)',
            overflow: 'hidden',
            backgroundColor: 'transparent',
            borderRadius: 'var(--rounded-box, 1rem)',
            ':focus': {
              outline: 'none',
            },
            ':focus-visible::-webkit-slider-thumb': {
              '--focus-shadow':
                '0 0 0 6px hsl(var(--b1)) inset, 0 0 0 2rem hsl(var(--range-shdw)) inset',
            },
            ':focus-visible::-moz-range-thumb': {
              '--focus-shadow':
                '0 0 0 6px hsl(var(--b1)) inset, 0 0 0 2rem hsl(var(--range-shdw)) inset',
            },
            '::-webkit-slider-runnable-track': {
              height: '0.5rem',
              width: '100%',
              borderRadius: 'var(--rounded-box, 1rem)',
              backgroundColor: 'hsla(var(--bc) / 0.1)',
            },
            '::-moz-range-track': {
              height: '0.5rem',
              width: '100%',
              borderRadius: 'var(--rounded-box, 1rem)',
              backgroundColor: 'hsla(var(--bc) / 0.1)',
            },
            '::-webkit-slider-thumb': {
              backgroundColor: 'hsl(var(--b1))',
              position: 'relative',
              height: '1.5rem',
              width: '1.5rem',
              borderStyle: 'none',
              borderRadius: 'var(--rounded-box, 1rem)',
              WebkitAppearance: 'none',
              top: '50%',
              color: 'hsl(var(--range-shdw))',
              transform: 'translateY(-50%)',
              '--filler-size': '100rem',
              '--filler-offset': '0.6rem',
              boxShadow:
                '0 0 0 3px hsl(var(--range-shdw)) inset, var(--focus-shadow, 0 0), calc(var(--filler-size) * -1 - var(--filler-offset)) 0 0 var(--filler-size)',
            },
            '::-moz-range-thumb': {
              backgroundColor: 'hsl(var(--b1))',
              position: 'relative',
              height: '1.5rem',
              width: '1.5rem',
              borderStyle: 'none',
              borderRadius: 'var(--rounded-box, 1rem)',
              top: '50%',
              color: 'hsl(var(--range-shdw))',
              '--filler-size': '100rem',
              '--filler-offset': '0.5rem',
              boxShadow:
                '0 0 0 3px hsl(var(--range-shdw)) inset, var(--focus-shadow, 0 0), calc(var(--filler-size) * -1 - var(--filler-offset)) 0 0 var(--filler-size)',
            },
          }}
        />
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            position: 'relative',
            display: 'inline-flex',
            '& :where(input)': {
              cursor: 'pointer',
              animation: 'rating-pop var(--animation-input, 0.25s) ease-out',
              height: '1.5rem',
              width: '1.5rem',
              backgroundColor: 'hsl(var(--bc) / var(--tw-bg-opacity))',
              '--tw-bg-opacity': '1',
            },
            '& input': {
              appearance: 'none',
              WebkitAppearance: 'none',
            },
            '& .rating-hidden': {
              width: '0.5rem',
              backgroundColor: 'transparent',
            },
            '& input:checked ~ input': {
              '--tw-bg-opacity': '0.2',
            },
            '& input:focus-visible': {
              transitionProperty: 'transform',
              transitionDuration: '300ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              transform: 'translateY(-0.125em)',
            },
            '& input:active:focus': {
              animation: 'none',
              transform: 'translateY(-0.125em)',
            },
            gap: '0.25rem',
          }}
        >
          <input
            type="radio"
            name="rating-3"
            css={{
              WebkitMaskSize: 'contain',
              maskSize: 'contain',
              WebkitMaskRepeat: 'no-repeat',
              maskRepeat: 'no-repeat',
              WebkitMaskPosition: 'center',
              maskPosition: 'center',
              WebkitMaskImage:
                'url(data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iMjAwcHgiIGhlaWdodD0iMTg1cHgiIHZpZXdCb3g9IjAgMCAyMDAgMTg1IiB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiPgogICAgPCEtLSBHZW5lcmF0b3I6IFNrZXRjaCA2MC4xICg4ODEzMykgLSBodHRwczovL3NrZXRjaC5jb20gLS0+CiAgICA8dGl0bGU+aGVhcnQ8L3RpdGxlPgogICAgPGRlc2M+Q3JlYXRlZCB3aXRoIFNrZXRjaC48L2Rlc2M+CiAgICA8ZyBpZD0iUGFnZS0xIiBzdHJva2U9Im5vbmUiIHN0cm9rZS13aWR0aD0iMSIgZmlsbD0ibm9uZSIgZmlsbC1ydWxlPSJldmVub2RkIj4KICAgICAgICA8cGF0aCBkPSJNMTAwLDE4NC42MDU1MzQgQzk2LjkxMjE3MTYsMTg0LjYwMTYzNSA5My44OTY5NzMzLDE4My42Njg1OTggOTEuMzQ2NjE4MiwxODEuOTI3NzkzIEM1My41NjQ5ODA0LDE1Ni4yODAxMjMgMzcuMjA1Mjc2NCwxMzguNjk0NTIzIDI4LjE4MTcxOTQsMTI3LjY5OTkxNyBDOC45NTE5NzYyNiwxMDQuMjYzNjY3IC0wLjI1NDI2MzI3Myw4MC4yMDI0NTEzIDAuMDA1MzM4MjU5MzEsNTQuMTQ2MTQ5MyBDMC4zMDgyMDY3MTQsMjQuMjg3MTY1NiAyNC4yNjM2NTkzLDAgNTMuNDA2MzM1LDAgQzc0LjU5NzUxMiwwIDg5LjI3NDYxMzQsMTEuOTM2ODYzMSA5Ny44MjIyMzQzLDIxLjg3ODY0MDMgQzk4LjM3MDA4MTIsMjIuNTA5NDMgOTkuMTY0NTE5NiwyMi44NzE2ODg5IDEwMCwyMi44NzE2ODg5IEMxMDAuODM1NDg2LDIyLjg3MTY4ODkgMTAxLjYyOTkyNCwyMi41MDk0MyAxMDIuMTc3NzcxLDIxLjg3ODY0MDMgQzExMC43MjUzOTIsMTEuOTI3MjQ4MiAxMjUuNDAyNDkzLDAgMTQ2LjU5MzY3LDAgQzE3NS43MzYzNDYsMCAxOTkuNjkxNzk5LDI0LjI4NzE2NTYgMTk5Ljk5NDY2Nyw1NC4xNTA5NTY3IEMyMDAuMjU0MjY5LDgwLjIxMjA2NjEgMTkxLjAzODQxNCwxMDQuMjczMjgyIDE3MS44MTgyODYsMTI3LjcwNDcyNCBDMTYyLjc5NDcyOSwxMzguNjk5MzMgMTQ2LjQzNTAyNSwxNTYuMjg0OTMgMTA4LjY1MzM4NywxODEuOTMyNiBDMTA2LjEwMjQ4NCwxODMuNjcxNzA0IDEwMy4wODczMjksMTg0LjYwMzA1MiAxMDAsMTg0LjYwNTUzNCBaIiBpZD0iaGVhcnQiIGZpbGw9IiMwMDAwMDAiIGZpbGwtcnVsZT0ibm9uemVybyI+PC9wYXRoPgogICAgPC9nPgo8L3N2Zz4=)',
              maskImage:
                'url(data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iMjAwcHgiIGhlaWdodD0iMTg1cHgiIHZpZXdCb3g9IjAgMCAyMDAgMTg1IiB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiPgogICAgPCEtLSBHZW5lcmF0b3I6IFNrZXRjaCA2MC4xICg4ODEzMykgLSBodHRwczovL3NrZXRjaC5jb20gLS0+CiAgICA8dGl0bGU+aGVhcnQ8L3RpdGxlPgogICAgPGRlc2M+Q3JlYXRlZCB3aXRoIFNrZXRjaC48L2Rlc2M+CiAgICA8ZyBpZD0iUGFnZS0xIiBzdHJva2U9Im5vbmUiIHN0cm9rZS13aWR0aD0iMSIgZmlsbD0ibm9uZSIgZmlsbC1ydWxlPSJldmVub2RkIj4KICAgICAgICA8cGF0aCBkPSJNMTAwLDE4NC42MDU1MzQgQzk2LjkxMjE3MTYsMTg0LjYwMTYzNSA5My44OTY5NzMzLDE4My42Njg1OTggOTEuMzQ2NjE4MiwxODEuOTI3NzkzIEM1My41NjQ5ODA0LDE1Ni4yODAxMjMgMzcuMjA1Mjc2NCwxMzguNjk0NTIzIDI4LjE4MTcxOTQsMTI3LjY5OTkxNyBDOC45NTE5NzYyNiwxMDQuMjYzNjY3IC0wLjI1NDI2MzI3Myw4MC4yMDI0NTEzIDAuMDA1MzM4MjU5MzEsNTQuMTQ2MTQ5MyBDMC4zMDgyMDY3MTQsMjQuMjg3MTY1NiAyNC4yNjM2NTkzLDAgNTMuNDA2MzM1LDAgQzc0LjU5NzUxMiwwIDg5LjI3NDYxMzQsMTEuOTM2ODYzMSA5Ny44MjIyMzQzLDIxLjg3ODY0MDMgQzk4LjM3MDA4MTIsMjIuNTA5NDMgOTkuMTY0NTE5NiwyMi44NzE2ODg5IDEwMCwyMi44NzE2ODg5IEMxMDAuODM1NDg2LDIyLjg3MTY4ODkgMTAxLjYyOTkyNCwyMi41MDk0MyAxMDIuMTc3NzcxLDIxLjg3ODY0MDMgQzExMC43MjUzOTIsMTEuOTI3MjQ4MiAxMjUuNDAyNDkzLDAgMTQ2LjU5MzY3LDAgQzE3NS43MzYzNDYsMCAxOTkuNjkxNzk5LDI0LjI4NzE2NTYgMTk5Ljk5NDY2Nyw1NC4xNTA5NTY3IEMyMDAuMjU0MjY5LDgwLjIxMjA2NjEgMTkxLjAzODQxNCwxMDQuMjczMjgyIDE3MS44MTgyODYsMTI3LjcwNDcyNCBDMTYyLjc5NDcyOSwxMzguNjk5MzMgMTQ2LjQzNTAyNSwxNTYuMjg0OTMgMTA4LjY1MzM4NywxODEuOTMyNiBDMTA2LjEwMjQ4NCwxODMuNjcxNzA0IDEwMy4wODczMjksMTg0LjYwMzA1MiAxMDAsMTg0LjYwNTUzNCBaIiBpZD0iaGVhcnQiIGZpbGw9IiMwMDAwMDAiIGZpbGwtcnVsZT0ibm9uemVybyI+PC9wYXRoPgogICAgPC9nPgo8L3N2Zz4=)',
              '--tw-bg-opacity': '1',
              backgroundColor: 'rgb(248 113 113 / var(--tw-bg-opacity))',
            }}
          />
          <input
            type="radio"
            name="rating-3"
            css={{
              WebkitMaskSize: 'contain',
              maskSize: 'contain',
              WebkitMaskRepeat: 'no-repeat',
              maskRepeat: 'no-repeat',
              WebkitMaskPosition: 'center',
              maskPosition: 'center',
              WebkitMaskImage:
                'url(data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iMjAwcHgiIGhlaWdodD0iMTg1cHgiIHZpZXdCb3g9IjAgMCAyMDAgMTg1IiB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiPgogICAgPCEtLSBHZW5lcmF0b3I6IFNrZXRjaCA2MC4xICg4ODEzMykgLSBodHRwczovL3NrZXRjaC5jb20gLS0+CiAgICA8dGl0bGU+aGVhcnQ8L3RpdGxlPgogICAgPGRlc2M+Q3JlYXRlZCB3aXRoIFNrZXRjaC48L2Rlc2M+CiAgICA8ZyBpZD0iUGFnZS0xIiBzdHJva2U9Im5vbmUiIHN0cm9rZS13aWR0aD0iMSIgZmlsbD0ibm9uZSIgZmlsbC1ydWxlPSJldmVub2RkIj4KICAgICAgICA8cGF0aCBkPSJNMTAwLDE4NC42MDU1MzQgQzk2LjkxMjE3MTYsMTg0LjYwMTYzNSA5My44OTY5NzMzLDE4My42Njg1OTggOTEuMzQ2NjE4MiwxODEuOTI3NzkzIEM1My41NjQ5ODA0LDE1Ni4yODAxMjMgMzcuMjA1Mjc2NCwxMzguNjk0NTIzIDI4LjE4MTcxOTQsMTI3LjY5OTkxNyBDOC45NTE5NzYyNiwxMDQuMjYzNjY3IC0wLjI1NDI2MzI3Myw4MC4yMDI0NTEzIDAuMDA1MzM4MjU5MzEsNTQuMTQ2MTQ5MyBDMC4zMDgyMDY3MTQsMjQuMjg3MTY1NiAyNC4yNjM2NTkzLDAgNTMuNDA2MzM1LDAgQzc0LjU5NzUxMiwwIDg5LjI3NDYxMzQsMTEuOTM2ODYzMSA5Ny44MjIyMzQzLDIxLjg3ODY0MDMgQzk4LjM3MDA4MTIsMjIuNTA5NDMgOTkuMTY0NTE5NiwyMi44NzE2ODg5IDEwMCwyMi44NzE2ODg5IEMxMDAuODM1NDg2LDIyLjg3MTY4ODkgMTAxLjYyOTkyNCwyMi41MDk0MyAxMDIuMTc3NzcxLDIxLjg3ODY0MDMgQzExMC43MjUzOTIsMTEuOTI3MjQ4MiAxMjUuNDAyNDkzLDAgMTQ2LjU5MzY3LDAgQzE3NS43MzYzNDYsMCAxOTkuNjkxNzk5LDI0LjI4NzE2NTYgMTk5Ljk5NDY2Nyw1NC4xNTA5NTY3IEMyMDAuMjU0MjY5LDgwLjIxMjA2NjEgMTkxLjAzODQxNCwxMDQuMjczMjgyIDE3MS44MTgyODYsMTI3LjcwNDcyNCBDMTYyLjc5NDcyOSwxMzguNjk5MzMgMTQ2LjQzNTAyNSwxNTYuMjg0OTMgMTA4LjY1MzM4NywxODEuOTMyNiBDMTA2LjEwMjQ4NCwxODMuNjcxNzA0IDEwMy4wODczMjksMTg0LjYwMzA1MiAxMDAsMTg0LjYwNTUzNCBaIiBpZD0iaGVhcnQiIGZpbGw9IiMwMDAwMDAiIGZpbGwtcnVsZT0ibm9uemVybyI+PC9wYXRoPgogICAgPC9nPgo8L3N2Zz4=)',
              maskImage:
                'url(data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iMjAwcHgiIGhlaWdodD0iMTg1cHgiIHZpZXdCb3g9IjAgMCAyMDAgMTg1IiB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiPgogICAgPCEtLSBHZW5lcmF0b3I6IFNrZXRjaCA2MC4xICg4ODEzMykgLSBodHRwczovL3NrZXRjaC5jb20gLS0+CiAgICA8dGl0bGU+aGVhcnQ8L3RpdGxlPgogICAgPGRlc2M+Q3JlYXRlZCB3aXRoIFNrZXRjaC48L2Rlc2M+CiAgICA8ZyBpZD0iUGFnZS0xIiBzdHJva2U9Im5vbmUiIHN0cm9rZS13aWR0aD0iMSIgZmlsbD0ibm9uZSIgZmlsbC1ydWxlPSJldmVub2RkIj4KICAgICAgICA8cGF0aCBkPSJNMTAwLDE4NC42MDU1MzQgQzk2LjkxMjE3MTYsMTg0LjYwMTYzNSA5My44OTY5NzMzLDE4My42Njg1OTggOTEuMzQ2NjE4MiwxODEuOTI3NzkzIEM1My41NjQ5ODA0LDE1Ni4yODAxMjMgMzcuMjA1Mjc2NCwxMzguNjk0NTIzIDI4LjE4MTcxOTQsMTI3LjY5OTkxNyBDOC45NTE5NzYyNiwxMDQuMjYzNjY3IC0wLjI1NDI2MzI3Myw4MC4yMDI0NTEzIDAuMDA1MzM4MjU5MzEsNTQuMTQ2MTQ5MyBDMC4zMDgyMDY3MTQsMjQuMjg3MTY1NiAyNC4yNjM2NTkzLDAgNTMuNDA2MzM1LDAgQzc0LjU5NzUxMiwwIDg5LjI3NDYxMzQsMTEuOTM2ODYzMSA5Ny44MjIyMzQzLDIxLjg3ODY0MDMgQzk4LjM3MDA4MTIsMjIuNTA5NDMgOTkuMTY0NTE5NiwyMi44NzE2ODg5IDEwMCwyMi44NzE2ODg5IEMxMDAuODM1NDg2LDIyLjg3MTY4ODkgMTAxLjYyOTkyNCwyMi41MDk0MyAxMDIuMTc3NzcxLDIxLjg3ODY0MDMgQzExMC43MjUzOTIsMTEuOTI3MjQ4MiAxMjUuNDAyNDkzLDAgMTQ2LjU5MzY3LDAgQzE3NS43MzYzNDYsMCAxOTkuNjkxNzk5LDI0LjI4NzE2NTYgMTk5Ljk5NDY2Nyw1NC4xNTA5NTY3IEMyMDAuMjU0MjY5LDgwLjIxMjA2NjEgMTkxLjAzODQxNCwxMDQuMjczMjgyIDE3MS44MTgyODYsMTI3LjcwNDcyNCBDMTYyLjc5NDcyOSwxMzguNjk5MzMgMTQ2LjQzNTAyNSwxNTYuMjg0OTMgMTA4LjY1MzM4NywxODEuOTMyNiBDMTA2LjEwMjQ4NCwxODMuNjcxNzA0IDEwMy4wODczMjksMTg0LjYwMzA1MiAxMDAsMTg0LjYwNTUzNCBaIiBpZD0iaGVhcnQiIGZpbGw9IiMwMDAwMDAiIGZpbGwtcnVsZT0ibm9uemVybyI+PC9wYXRoPgogICAgPC9nPgo8L3N2Zz4=)',
              '--tw-bg-opacity': '1',
              backgroundColor: 'rgb(251 146 60 / var(--tw-bg-opacity))',
            }}
          />
          <input
            type="radio"
            name="rating-3"
            css={{
              WebkitMaskSize: 'contain',
              maskSize: 'contain',
              WebkitMaskRepeat: 'no-repeat',
              maskRepeat: 'no-repeat',
              WebkitMaskPosition: 'center',
              maskPosition: 'center',
              WebkitMaskImage:
                'url(data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iMjAwcHgiIGhlaWdodD0iMTg1cHgiIHZpZXdCb3g9IjAgMCAyMDAgMTg1IiB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiPgogICAgPCEtLSBHZW5lcmF0b3I6IFNrZXRjaCA2MC4xICg4ODEzMykgLSBodHRwczovL3NrZXRjaC5jb20gLS0+CiAgICA8dGl0bGU+aGVhcnQ8L3RpdGxlPgogICAgPGRlc2M+Q3JlYXRlZCB3aXRoIFNrZXRjaC48L2Rlc2M+CiAgICA8ZyBpZD0iUGFnZS0xIiBzdHJva2U9Im5vbmUiIHN0cm9rZS13aWR0aD0iMSIgZmlsbD0ibm9uZSIgZmlsbC1ydWxlPSJldmVub2RkIj4KICAgICAgICA8cGF0aCBkPSJNMTAwLDE4NC42MDU1MzQgQzk2LjkxMjE3MTYsMTg0LjYwMTYzNSA5My44OTY5NzMzLDE4My42Njg1OTggOTEuMzQ2NjE4MiwxODEuOTI3NzkzIEM1My41NjQ5ODA0LDE1Ni4yODAxMjMgMzcuMjA1Mjc2NCwxMzguNjk0NTIzIDI4LjE4MTcxOTQsMTI3LjY5OTkxNyBDOC45NTE5NzYyNiwxMDQuMjYzNjY3IC0wLjI1NDI2MzI3Myw4MC4yMDI0NTEzIDAuMDA1MzM4MjU5MzEsNTQuMTQ2MTQ5MyBDMC4zMDgyMDY3MTQsMjQuMjg3MTY1NiAyNC4yNjM2NTkzLDAgNTMuNDA2MzM1LDAgQzc0LjU5NzUxMiwwIDg5LjI3NDYxMzQsMTEuOTM2ODYzMSA5Ny44MjIyMzQzLDIxLjg3ODY0MDMgQzk4LjM3MDA4MTIsMjIuNTA5NDMgOTkuMTY0NTE5NiwyMi44NzE2ODg5IDEwMCwyMi44NzE2ODg5IEMxMDAuODM1NDg2LDIyLjg3MTY4ODkgMTAxLjYyOTkyNCwyMi41MDk0MyAxMDIuMTc3NzcxLDIxLjg3ODY0MDMgQzExMC43MjUzOTIsMTEuOTI3MjQ4MiAxMjUuNDAyNDkzLDAgMTQ2LjU5MzY3LDAgQzE3NS43MzYzNDYsMCAxOTkuNjkxNzk5LDI0LjI4NzE2NTYgMTk5Ljk5NDY2Nyw1NC4xNTA5NTY3IEMyMDAuMjU0MjY5LDgwLjIxMjA2NjEgMTkxLjAzODQxNCwxMDQuMjczMjgyIDE3MS44MTgyODYsMTI3LjcwNDcyNCBDMTYyLjc5NDcyOSwxMzguNjk5MzMgMTQ2LjQzNTAyNSwxNTYuMjg0OTMgMTA4LjY1MzM4NywxODEuOTMyNiBDMTA2LjEwMjQ4NCwxODMuNjcxNzA0IDEwMy4wODczMjksMTg0LjYwMzA1MiAxMDAsMTg0LjYwNTUzNCBaIiBpZD0iaGVhcnQiIGZpbGw9IiMwMDAwMDAiIGZpbGwtcnVsZT0ibm9uemVybyI+PC9wYXRoPgogICAgPC9nPgo8L3N2Zz4=)',
              maskImage:
                'url(data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iMjAwcHgiIGhlaWdodD0iMTg1cHgiIHZpZXdCb3g9IjAgMCAyMDAgMTg1IiB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiPgogICAgPCEtLSBHZW5lcmF0b3I6IFNrZXRjaCA2MC4xICg4ODEzMykgLSBodHRwczovL3NrZXRjaC5jb20gLS0+CiAgICA8dGl0bGU+aGVhcnQ8L3RpdGxlPgogICAgPGRlc2M+Q3JlYXRlZCB3aXRoIFNrZXRjaC48L2Rlc2M+CiAgICA8ZyBpZD0iUGFnZS0xIiBzdHJva2U9Im5vbmUiIHN0cm9rZS13aWR0aD0iMSIgZmlsbD0ibm9uZSIgZmlsbC1ydWxlPSJldmVub2RkIj4KICAgICAgICA8cGF0aCBkPSJNMTAwLDE4NC42MDU1MzQgQzk2LjkxMjE3MTYsMTg0LjYwMTYzNSA5My44OTY5NzMzLDE4My42Njg1OTggOTEuMzQ2NjE4MiwxODEuOTI3NzkzIEM1My41NjQ5ODA0LDE1Ni4yODAxMjMgMzcuMjA1Mjc2NCwxMzguNjk0NTIzIDI4LjE4MTcxOTQsMTI3LjY5OTkxNyBDOC45NTE5NzYyNiwxMDQuMjYzNjY3IC0wLjI1NDI2MzI3Myw4MC4yMDI0NTEzIDAuMDA1MzM4MjU5MzEsNTQuMTQ2MTQ5MyBDMC4zMDgyMDY3MTQsMjQuMjg3MTY1NiAyNC4yNjM2NTkzLDAgNTMuNDA2MzM1LDAgQzc0LjU5NzUxMiwwIDg5LjI3NDYxMzQsMTEuOTM2ODYzMSA5Ny44MjIyMzQzLDIxLjg3ODY0MDMgQzk4LjM3MDA4MTIsMjIuNTA5NDMgOTkuMTY0NTE5NiwyMi44NzE2ODg5IDEwMCwyMi44NzE2ODg5IEMxMDAuODM1NDg2LDIyLjg3MTY4ODkgMTAxLjYyOTkyNCwyMi41MDk0MyAxMDIuMTc3NzcxLDIxLjg3ODY0MDMgQzExMC43MjUzOTIsMTEuOTI3MjQ4MiAxMjUuNDAyNDkzLDAgMTQ2LjU5MzY3LDAgQzE3NS43MzYzNDYsMCAxOTkuNjkxNzk5LDI0LjI4NzE2NTYgMTk5Ljk5NDY2Nyw1NC4xNTA5NTY3IEMyMDAuMjU0MjY5LDgwLjIxMjA2NjEgMTkxLjAzODQxNCwxMDQuMjczMjgyIDE3MS44MTgyODYsMTI3LjcwNDcyNCBDMTYyLjc5NDcyOSwxMzguNjk5MzMgMTQ2LjQzNTAyNSwxNTYuMjg0OTMgMTA4LjY1MzM4NywxODEuOTMyNiBDMTA2LjEwMjQ4NCwxODMuNjcxNzA0IDEwMy4wODczMjksMTg0LjYwMzA1MiAxMDAsMTg0LjYwNTUzNCBaIiBpZD0iaGVhcnQiIGZpbGw9IiMwMDAwMDAiIGZpbGwtcnVsZT0ibm9uemVybyI+PC9wYXRoPgogICAgPC9nPgo8L3N2Zz4=)',
              '--tw-bg-opacity': '1',
              backgroundColor: 'rgb(250 204 21 / var(--tw-bg-opacity))',
            }}
          />
          <input
            type="radio"
            name="rating-3"
            css={{
              WebkitMaskSize: 'contain',
              maskSize: 'contain',
              WebkitMaskRepeat: 'no-repeat',
              maskRepeat: 'no-repeat',
              WebkitMaskPosition: 'center',
              maskPosition: 'center',
              WebkitMaskImage:
                'url(data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iMjAwcHgiIGhlaWdodD0iMTg1cHgiIHZpZXdCb3g9IjAgMCAyMDAgMTg1IiB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiPgogICAgPCEtLSBHZW5lcmF0b3I6IFNrZXRjaCA2MC4xICg4ODEzMykgLSBodHRwczovL3NrZXRjaC5jb20gLS0+CiAgICA8dGl0bGU+aGVhcnQ8L3RpdGxlPgogICAgPGRlc2M+Q3JlYXRlZCB3aXRoIFNrZXRjaC48L2Rlc2M+CiAgICA8ZyBpZD0iUGFnZS0xIiBzdHJva2U9Im5vbmUiIHN0cm9rZS13aWR0aD0iMSIgZmlsbD0ibm9uZSIgZmlsbC1ydWxlPSJldmVub2RkIj4KICAgICAgICA8cGF0aCBkPSJNMTAwLDE4NC42MDU1MzQgQzk2LjkxMjE3MTYsMTg0LjYwMTYzNSA5My44OTY5NzMzLDE4My42Njg1OTggOTEuMzQ2NjE4MiwxODEuOTI3NzkzIEM1My41NjQ5ODA0LDE1Ni4yODAxMjMgMzcuMjA1Mjc2NCwxMzguNjk0NTIzIDI4LjE4MTcxOTQsMTI3LjY5OTkxNyBDOC45NTE5NzYyNiwxMDQuMjYzNjY3IC0wLjI1NDI2MzI3Myw4MC4yMDI0NTEzIDAuMDA1MzM4MjU5MzEsNTQuMTQ2MTQ5MyBDMC4zMDgyMDY3MTQsMjQuMjg3MTY1NiAyNC4yNjM2NTkzLDAgNTMuNDA2MzM1LDAgQzc0LjU5NzUxMiwwIDg5LjI3NDYxMzQsMTEuOTM2ODYzMSA5Ny44MjIyMzQzLDIxLjg3ODY0MDMgQzk4LjM3MDA4MTIsMjIuNTA5NDMgOTkuMTY0NTE5NiwyMi44NzE2ODg5IDEwMCwyMi44NzE2ODg5IEMxMDAuODM1NDg2LDIyLjg3MTY4ODkgMTAxLjYyOTkyNCwyMi41MDk0MyAxMDIuMTc3NzcxLDIxLjg3ODY0MDMgQzExMC43MjUzOTIsMTEuOTI3MjQ4MiAxMjUuNDAyNDkzLDAgMTQ2LjU5MzY3LDAgQzE3NS43MzYzNDYsMCAxOTkuNjkxNzk5LDI0LjI4NzE2NTYgMTk5Ljk5NDY2Nyw1NC4xNTA5NTY3IEMyMDAuMjU0MjY5LDgwLjIxMjA2NjEgMTkxLjAzODQxNCwxMDQuMjczMjgyIDE3MS44MTgyODYsMTI3LjcwNDcyNCBDMTYyLjc5NDcyOSwxMzguNjk5MzMgMTQ2LjQzNTAyNSwxNTYuMjg0OTMgMTA4LjY1MzM4NywxODEuOTMyNiBDMTA2LjEwMjQ4NCwxODMuNjcxNzA0IDEwMy4wODczMjksMTg0LjYwMzA1MiAxMDAsMTg0LjYwNTUzNCBaIiBpZD0iaGVhcnQiIGZpbGw9IiMwMDAwMDAiIGZpbGwtcnVsZT0ibm9uemVybyI+PC9wYXRoPgogICAgPC9nPgo8L3N2Zz4=)',
              maskImage:
                'url(data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iMjAwcHgiIGhlaWdodD0iMTg1cHgiIHZpZXdCb3g9IjAgMCAyMDAgMTg1IiB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiPgogICAgPCEtLSBHZW5lcmF0b3I6IFNrZXRjaCA2MC4xICg4ODEzMykgLSBodHRwczovL3NrZXRjaC5jb20gLS0+CiAgICA8dGl0bGU+aGVhcnQ8L3RpdGxlPgogICAgPGRlc2M+Q3JlYXRlZCB3aXRoIFNrZXRjaC48L2Rlc2M+CiAgICA8ZyBpZD0iUGFnZS0xIiBzdHJva2U9Im5vbmUiIHN0cm9rZS13aWR0aD0iMSIgZmlsbD0ibm9uZSIgZmlsbC1ydWxlPSJldmVub2RkIj4KICAgICAgICA8cGF0aCBkPSJNMTAwLDE4NC42MDU1MzQgQzk2LjkxMjE3MTYsMTg0LjYwMTYzNSA5My44OTY5NzMzLDE4My42Njg1OTggOTEuMzQ2NjE4MiwxODEuOTI3NzkzIEM1My41NjQ5ODA0LDE1Ni4yODAxMjMgMzcuMjA1Mjc2NCwxMzguNjk0NTIzIDI4LjE4MTcxOTQsMTI3LjY5OTkxNyBDOC45NTE5NzYyNiwxMDQuMjYzNjY3IC0wLjI1NDI2MzI3Myw4MC4yMDI0NTEzIDAuMDA1MzM4MjU5MzEsNTQuMTQ2MTQ5MyBDMC4zMDgyMDY3MTQsMjQuMjg3MTY1NiAyNC4yNjM2NTkzLDAgNTMuNDA2MzM1LDAgQzc0LjU5NzUxMiwwIDg5LjI3NDYxMzQsMTEuOTM2ODYzMSA5Ny44MjIyMzQzLDIxLjg3ODY0MDMgQzk4LjM3MDA4MTIsMjIuNTA5NDMgOTkuMTY0NTE5NiwyMi44NzE2ODg5IDEwMCwyMi44NzE2ODg5IEMxMDAuODM1NDg2LDIyLjg3MTY4ODkgMTAxLjYyOTkyNCwyMi41MDk0MyAxMDIuMTc3NzcxLDIxLjg3ODY0MDMgQzExMC43MjUzOTIsMTEuOTI3MjQ4MiAxMjUuNDAyNDkzLDAgMTQ2LjU5MzY3LDAgQzE3NS43MzYzNDYsMCAxOTkuNjkxNzk5LDI0LjI4NzE2NTYgMTk5Ljk5NDY2Nyw1NC4xNTA5NTY3IEMyMDAuMjU0MjY5LDgwLjIxMjA2NjEgMTkxLjAzODQxNCwxMDQuMjczMjgyIDE3MS44MTgyODYsMTI3LjcwNDcyNCBDMTYyLjc5NDcyOSwxMzguNjk5MzMgMTQ2LjQzNTAyNSwxNTYuMjg0OTMgMTA4LjY1MzM4NywxODEuOTMyNiBDMTA2LjEwMjQ4NCwxODMuNjcxNzA0IDEwMy4wODczMjksMTg0LjYwMzA1MiAxMDAsMTg0LjYwNTUzNCBaIiBpZD0iaGVhcnQiIGZpbGw9IiMwMDAwMDAiIGZpbGwtcnVsZT0ibm9uemVybyI+PC9wYXRoPgogICAgPC9nPgo8L3N2Zz4=)',
              '--tw-bg-opacity': '1',
              backgroundColor: 'rgb(163 230 53 / var(--tw-bg-opacity))',
            }}
          />
          <input
            type="radio"
            name="rating-3"
            css={{
              WebkitMaskSize: 'contain',
              maskSize: 'contain',
              WebkitMaskRepeat: 'no-repeat',
              maskRepeat: 'no-repeat',
              WebkitMaskPosition: 'center',
              maskPosition: 'center',
              WebkitMaskImage:
                'url(data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iMjAwcHgiIGhlaWdodD0iMTg1cHgiIHZpZXdCb3g9IjAgMCAyMDAgMTg1IiB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiPgogICAgPCEtLSBHZW5lcmF0b3I6IFNrZXRjaCA2MC4xICg4ODEzMykgLSBodHRwczovL3NrZXRjaC5jb20gLS0+CiAgICA8dGl0bGU+aGVhcnQ8L3RpdGxlPgogICAgPGRlc2M+Q3JlYXRlZCB3aXRoIFNrZXRjaC48L2Rlc2M+CiAgICA8ZyBpZD0iUGFnZS0xIiBzdHJva2U9Im5vbmUiIHN0cm9rZS13aWR0aD0iMSIgZmlsbD0ibm9uZSIgZmlsbC1ydWxlPSJldmVub2RkIj4KICAgICAgICA8cGF0aCBkPSJNMTAwLDE4NC42MDU1MzQgQzk2LjkxMjE3MTYsMTg0LjYwMTYzNSA5My44OTY5NzMzLDE4My42Njg1OTggOTEuMzQ2NjE4MiwxODEuOTI3NzkzIEM1My41NjQ5ODA0LDE1Ni4yODAxMjMgMzcuMjA1Mjc2NCwxMzguNjk0NTIzIDI4LjE4MTcxOTQsMTI3LjY5OTkxNyBDOC45NTE5NzYyNiwxMDQuMjYzNjY3IC0wLjI1NDI2MzI3Myw4MC4yMDI0NTEzIDAuMDA1MzM4MjU5MzEsNTQuMTQ2MTQ5MyBDMC4zMDgyMDY3MTQsMjQuMjg3MTY1NiAyNC4yNjM2NTkzLDAgNTMuNDA2MzM1LDAgQzc0LjU5NzUxMiwwIDg5LjI3NDYxMzQsMTEuOTM2ODYzMSA5Ny44MjIyMzQzLDIxLjg3ODY0MDMgQzk4LjM3MDA4MTIsMjIuNTA5NDMgOTkuMTY0NTE5NiwyMi44NzE2ODg5IDEwMCwyMi44NzE2ODg5IEMxMDAuODM1NDg2LDIyLjg3MTY4ODkgMTAxLjYyOTkyNCwyMi41MDk0MyAxMDIuMTc3NzcxLDIxLjg3ODY0MDMgQzExMC43MjUzOTIsMTEuOTI3MjQ4MiAxMjUuNDAyNDkzLDAgMTQ2LjU5MzY3LDAgQzE3NS43MzYzNDYsMCAxOTkuNjkxNzk5LDI0LjI4NzE2NTYgMTk5Ljk5NDY2Nyw1NC4xNTA5NTY3IEMyMDAuMjU0MjY5LDgwLjIxMjA2NjEgMTkxLjAzODQxNCwxMDQuMjczMjgyIDE3MS44MTgyODYsMTI3LjcwNDcyNCBDMTYyLjc5NDcyOSwxMzguNjk5MzMgMTQ2LjQzNTAyNSwxNTYuMjg0OTMgMTA4LjY1MzM4NywxODEuOTMyNiBDMTA2LjEwMjQ4NCwxODMuNjcxNzA0IDEwMy4wODczMjksMTg0LjYwMzA1MiAxMDAsMTg0LjYwNTUzNCBaIiBpZD0iaGVhcnQiIGZpbGw9IiMwMDAwMDAiIGZpbGwtcnVsZT0ibm9uemVybyI+PC9wYXRoPgogICAgPC9nPgo8L3N2Zz4=)',
              maskImage:
                'url(data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iMjAwcHgiIGhlaWdodD0iMTg1cHgiIHZpZXdCb3g9IjAgMCAyMDAgMTg1IiB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiPgogICAgPCEtLSBHZW5lcmF0b3I6IFNrZXRjaCA2MC4xICg4ODEzMykgLSBodHRwczovL3NrZXRjaC5jb20gLS0+CiAgICA8dGl0bGU+aGVhcnQ8L3RpdGxlPgogICAgPGRlc2M+Q3JlYXRlZCB3aXRoIFNrZXRjaC48L2Rlc2M+CiAgICA8ZyBpZD0iUGFnZS0xIiBzdHJva2U9Im5vbmUiIHN0cm9rZS13aWR0aD0iMSIgZmlsbD0ibm9uZSIgZmlsbC1ydWxlPSJldmVub2RkIj4KICAgICAgICA8cGF0aCBkPSJNMTAwLDE4NC42MDU1MzQgQzk2LjkxMjE3MTYsMTg0LjYwMTYzNSA5My44OTY5NzMzLDE4My42Njg1OTggOTEuMzQ2NjE4MiwxODEuOTI3NzkzIEM1My41NjQ5ODA0LDE1Ni4yODAxMjMgMzcuMjA1Mjc2NCwxMzguNjk0NTIzIDI4LjE4MTcxOTQsMTI3LjY5OTkxNyBDOC45NTE5NzYyNiwxMDQuMjYzNjY3IC0wLjI1NDI2MzI3Myw4MC4yMDI0NTEzIDAuMDA1MzM4MjU5MzEsNTQuMTQ2MTQ5MyBDMC4zMDgyMDY3MTQsMjQuMjg3MTY1NiAyNC4yNjM2NTkzLDAgNTMuNDA2MzM1LDAgQzc0LjU5NzUxMiwwIDg5LjI3NDYxMzQsMTEuOTM2ODYzMSA5Ny44MjIyMzQzLDIxLjg3ODY0MDMgQzk4LjM3MDA4MTIsMjIuNTA5NDMgOTkuMTY0NTE5NiwyMi44NzE2ODg5IDEwMCwyMi44NzE2ODg5IEMxMDAuODM1NDg2LDIyLjg3MTY4ODkgMTAxLjYyOTkyNCwyMi41MDk0MyAxMDIuMTc3NzcxLDIxLjg3ODY0MDMgQzExMC43MjUzOTIsMTEuOTI3MjQ4MiAxMjUuNDAyNDkzLDAgMTQ2LjU5MzY3LDAgQzE3NS43MzYzNDYsMCAxOTkuNjkxNzk5LDI0LjI4NzE2NTYgMTk5Ljk5NDY2Nyw1NC4xNTA5NTY3IEMyMDAuMjU0MjY5LDgwLjIxMjA2NjEgMTkxLjAzODQxNCwxMDQuMjczMjgyIDE3MS44MTgyODYsMTI3LjcwNDcyNCBDMTYyLjc5NDcyOSwxMzguNjk5MzMgMTQ2LjQzNTAyNSwxNTYuMjg0OTMgMTA4LjY1MzM4NywxODEuOTMyNiBDMTA2LjEwMjQ4NCwxODMuNjcxNzA0IDEwMy4wODczMjksMTg0LjYwMzA1MiAxMDAsMTg0LjYwNTUzNCBaIiBpZD0iaGVhcnQiIGZpbGw9IiMwMDAwMDAiIGZpbGwtcnVsZT0ibm9uemVybyI+PC9wYXRoPgogICAgPC9nPgo8L3N2Zz4=)',
              '--tw-bg-opacity': '1',
              backgroundColor: 'rgb(74 222 128 / var(--tw-bg-opacity))',
            }}
          />
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <select
          css={{
            display: 'inline-flex',
            flexShrink: '0',
            cursor: 'pointer',
            userSelect: 'none',
            appearance: 'none',
            height: '3rem',
            paddingLeft: '1rem',
            paddingRight: '2.5rem',
            fontSize: '0.875rem',
            lineHeight: '2',
            minHeight: '3rem',
            borderWidth: '1px',
            borderColor: 'hsl(var(--su) / var(--tw-border-opacity))',
            '--tw-border-opacity': '1',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--b1) / var(--tw-bg-opacity))',
            fontWeight: '600',
            transitionProperty:
              'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
            transitionDuration: '200ms',
            transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
            borderRadius: 'var(--rounded-btn, 0.5rem)',
            backgroundImage:
              'linear-gradient(45deg, transparent 50%, currentColor 50%), linear-gradient(135deg, currentColor 50%, transparent 50%)',
            backgroundPosition:
              'calc(100% - 20px) calc(1px + 50%), calc(100% - 16px) calc(1px + 50%)',
            backgroundSize: '4px 4px, 4px 4px',
            backgroundRepeat: 'no-repeat',
            ':focus': {
              outline: '2px solid hsl(var(--su))',
              outlineOffset: '2px',
            },
            width: '100%',
            maxWidth: '20rem',
          }}
        >
          <option disabled>Pick your favorite anime</option>
          <option>One Piece</option>
          <option>Naruto</option>
          <option>Death Note</option>
          <option>Attack on Titan</option>
          <option>Bleach</option>
          <option>Fullmetal Alchemist</option>
          <option>Jojo's Bizarre Adventure</option>
        </select>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <input
          type="checkbox"
          css={{
            flexShrink: '0',
            '--chkbg': 'hsla(var(--bc) / 0.2)',
            '--handleoffset': '1.5rem',
            height: '1.5rem',
            width: '3rem',
            cursor: 'pointer',
            appearance: 'none',
            borderWidth: '1px',
            borderColor: 'hsl(var(--bc) / var(--tw-border-opacity))',
            '--tw-border-opacity': '0.2',
            backgroundColor: 'hsl(var(--bc) / var(--tw-bg-opacity))',
            '--tw-bg-opacity': '0.2',
            transitionDuration: '300ms',
            transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
            borderRadius: 'var(--rounded-badge, 1.9rem)',
            transition:
              'background, box-shadow var(--animation-input, 0.2s) ease-in-out',
            boxShadow:
              'calc(var(--handleoffset) * -1) 0 0 2px hsl(var(--b1)) inset, 0 0 0 2px hsl(var(--b1)) inset',
            ':focus-visible': {
              outline: '2px solid hsl(var(--bc))',
              outlineOffset: '2px',
            },
            ':checked': {
              '--chkbg': 'hsl(var(--bc))',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              boxShadow:
                'var(--handleoffset) 0 0 2px hsl(var(--b1)) inset, 0 0 0 2px hsl(var(--b1)) inset',
            },
            '[dir="rtl"] &:checked': {
              boxShadow:
                'calc(var(--handleoffset) * 1) 0 0 2px hsl(var(--b1)) inset, 0 0 0 2px hsl(var(--b1)) inset',
            },
            ':indeterminate': {
              '--chkbg': 'hsl(var(--bc))',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              boxShadow:
                'calc(var(--handleoffset) / 2) 0 0 2px hsl(var(--b1)) inset, calc(var(--handleoffset) / -2) 0 0 2px hsl(var(--b1)) inset, 0 0 0 2px hsl(var(--b1)) inset',
            },
            '[dir="rtl"] &:indeterminate': {
              boxShadow:
                'calc(var(--handleoffset) / 2) 0 0 2px hsl(var(--b1)) inset, calc(var(--handleoffset) / -2) 0 0 2px hsl(var(--b1)) inset, 0 0 0 2px hsl(var(--b1)) inset',
            },
            ':disabled': {
              cursor: 'not-allowed',
              borderColor: 'transparent',
              backgroundColor: 'hsl(var(--bc) / var(--tw-bg-opacity))',
              '--tw-bg-opacity': '0.2',
            },
          }}
        />
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            '& > input[type="radio"].btn': {
              appearance: 'none',
            },
            '& > input[type="radio"].btn:before': {
              content: 'attr(data-title)',
            },
            '& > input[type="radio"]:checked.btn': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '& > .btn-active': {
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '& > input[type="radio"]:checked.btn:focus-visible': {
              outline: '2px solid hsl(var(--p))',
            },
            '& > .btn-active:focus-visible': {
              outline: '2px solid hsl(var(--p))',
            },
            '& > .btn:not(:first-of-type)': {
              marginLeft: '-1px',
              borderTopLeftRadius: '0px',
              borderBottomLeftRadius: '0px',
            },
            '& > .btn:not(:last-of-type)': {
              borderTopRightRadius: '0px',
              borderBottomRightRadius: '0px',
            },
            display: 'flex',
            flexWrap: 'wrap',
          }}
        >
          <input
            type="radio"
            name="options"
            data-title="1"
            css={{
              display: 'inline-flex',
              flexShrink: '0',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              borderColor: 'hsl(var(--n) / var(--tw-border-opacity))',
              textAlign: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              borderRadius: 'var(--rounded-btn, 0.5rem)',
              height: '3rem',
              paddingLeft: '1rem',
              paddingRight: '1rem',
              fontSize: '0.875rem',
              lineHeight: '1em',
              minHeight: '3rem',
              fontWeight: '600',
              textTransform: 'var(--btn-text-case, uppercase)',
              textDecorationLine: 'none',
              borderWidth: 'var(--border-btn, 1px)',
              animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              '&.loading': {
                pointerEvents: 'none',
              },
              '&.loading:hover': {
                pointerEvents: 'none',
              },
              '&.loading:before': {
                marginRight: '0.5rem',
                height: '1rem',
                width: '1rem',
                borderRadius: '9999px',
                borderWidth: '2px',
                animation: 'spin 2s linear infinite',
                content: '""',
                borderTopColor: 'transparent',
                borderLeftColor: 'transparent',
                borderBottomColor: 'currentColor',
                borderRightColor: 'currentColor',
              },
              ':active:hover': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':active:focus': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':hover': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
              },
              ':focus-visible': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '&.glass:hover': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass.btn-active': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass:focus-visible': {
                outline: '2px solid 0 0 2px currentColor',
              },
              '&.loading.btn-square:before': {
                marginRight: '0px',
              },
              '&.loading.btn-circle:before': {
                marginRight: '0px',
              },
              '&.loading.btn-xl:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-lg:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-sm:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '&.loading.btn-xs:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-group > input[type="radio"]:checked&': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-group > input[type="radio"]:checked&:focus-visible': {
                outline: '2px solid hsl(var(--p))',
              },
              '.btn-group > &:not(:first-of-type)': {
                marginLeft: '-1px',
                borderTopLeftRadius: '0px',
                borderBottomLeftRadius: '0px',
              },
              '.btn-group > &:not(:last-of-type)': {
                borderTopRightRadius: '0px',
                borderBottomRightRadius: '0px',
              },
              '@media (prefers-reduced-motion: reduce)': {
                '&.loading:before': {
                  animation: 'spin 10s linear infinite',
                },
              },
            }}
            className="btn"
          />
          <input
            type="radio"
            name="options"
            data-title="2"
            css={{
              display: 'inline-flex',
              flexShrink: '0',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              borderColor: 'hsl(var(--n) / var(--tw-border-opacity))',
              textAlign: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              borderRadius: 'var(--rounded-btn, 0.5rem)',
              height: '3rem',
              paddingLeft: '1rem',
              paddingRight: '1rem',
              fontSize: '0.875rem',
              lineHeight: '1em',
              minHeight: '3rem',
              fontWeight: '600',
              textTransform: 'var(--btn-text-case, uppercase)',
              textDecorationLine: 'none',
              borderWidth: 'var(--border-btn, 1px)',
              animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              '&.loading': {
                pointerEvents: 'none',
              },
              '&.loading:hover': {
                pointerEvents: 'none',
              },
              '&.loading:before': {
                marginRight: '0.5rem',
                height: '1rem',
                width: '1rem',
                borderRadius: '9999px',
                borderWidth: '2px',
                animation: 'spin 2s linear infinite',
                content: '""',
                borderTopColor: 'transparent',
                borderLeftColor: 'transparent',
                borderBottomColor: 'currentColor',
                borderRightColor: 'currentColor',
              },
              ':active:hover': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':active:focus': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':hover': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
              },
              ':focus-visible': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '&.glass:hover': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass.btn-active': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass:focus-visible': {
                outline: '2px solid 0 0 2px currentColor',
              },
              '&.loading.btn-square:before': {
                marginRight: '0px',
              },
              '&.loading.btn-circle:before': {
                marginRight: '0px',
              },
              '&.loading.btn-xl:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-lg:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-sm:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '&.loading.btn-xs:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-group > input[type="radio"]:checked&': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-group > input[type="radio"]:checked&:focus-visible': {
                outline: '2px solid hsl(var(--p))',
              },
              '.btn-group > &:not(:first-of-type)': {
                marginLeft: '-1px',
                borderTopLeftRadius: '0px',
                borderBottomLeftRadius: '0px',
              },
              '.btn-group > &:not(:last-of-type)': {
                borderTopRightRadius: '0px',
                borderBottomRightRadius: '0px',
              },
              '@media (prefers-reduced-motion: reduce)': {
                '&.loading:before': {
                  animation: 'spin 10s linear infinite',
                },
              },
            }}
            className="btn"
          />
          <input
            type="radio"
            name="options"
            data-title="3"
            css={{
              display: 'inline-flex',
              flexShrink: '0',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              borderColor: 'hsl(var(--n) / var(--tw-border-opacity))',
              textAlign: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              borderRadius: 'var(--rounded-btn, 0.5rem)',
              height: '3rem',
              paddingLeft: '1rem',
              paddingRight: '1rem',
              fontSize: '0.875rem',
              lineHeight: '1em',
              minHeight: '3rem',
              fontWeight: '600',
              textTransform: 'var(--btn-text-case, uppercase)',
              textDecorationLine: 'none',
              borderWidth: 'var(--border-btn, 1px)',
              animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              '&.loading': {
                pointerEvents: 'none',
              },
              '&.loading:hover': {
                pointerEvents: 'none',
              },
              '&.loading:before': {
                marginRight: '0.5rem',
                height: '1rem',
                width: '1rem',
                borderRadius: '9999px',
                borderWidth: '2px',
                animation: 'spin 2s linear infinite',
                content: '""',
                borderTopColor: 'transparent',
                borderLeftColor: 'transparent',
                borderBottomColor: 'currentColor',
                borderRightColor: 'currentColor',
              },
              ':active:hover': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':active:focus': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':hover': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
              },
              ':focus-visible': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '&.glass:hover': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass.btn-active': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass:focus-visible': {
                outline: '2px solid 0 0 2px currentColor',
              },
              '&.loading.btn-square:before': {
                marginRight: '0px',
              },
              '&.loading.btn-circle:before': {
                marginRight: '0px',
              },
              '&.loading.btn-xl:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-lg:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-sm:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '&.loading.btn-xs:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-group > input[type="radio"]:checked&': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-group > input[type="radio"]:checked&:focus-visible': {
                outline: '2px solid hsl(var(--p))',
              },
              '.btn-group > &:not(:first-of-type)': {
                marginLeft: '-1px',
                borderTopLeftRadius: '0px',
                borderBottomLeftRadius: '0px',
              },
              '.btn-group > &:not(:last-of-type)': {
                borderTopRightRadius: '0px',
                borderBottomRightRadius: '0px',
              },
              '@media (prefers-reduced-motion: reduce)': {
                '&.loading:before': {
                  animation: 'spin 10s linear infinite',
                },
              },
            }}
            className="btn"
          />
          <input
            type="radio"
            name="options"
            data-title="4"
            css={{
              display: 'inline-flex',
              flexShrink: '0',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              borderColor: 'hsl(var(--n) / var(--tw-border-opacity))',
              textAlign: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              borderRadius: 'var(--rounded-btn, 0.5rem)',
              height: '3rem',
              paddingLeft: '1rem',
              paddingRight: '1rem',
              fontSize: '0.875rem',
              lineHeight: '1em',
              minHeight: '3rem',
              fontWeight: '600',
              textTransform: 'var(--btn-text-case, uppercase)',
              textDecorationLine: 'none',
              borderWidth: 'var(--border-btn, 1px)',
              animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              '&.loading': {
                pointerEvents: 'none',
              },
              '&.loading:hover': {
                pointerEvents: 'none',
              },
              '&.loading:before': {
                marginRight: '0.5rem',
                height: '1rem',
                width: '1rem',
                borderRadius: '9999px',
                borderWidth: '2px',
                animation: 'spin 2s linear infinite',
                content: '""',
                borderTopColor: 'transparent',
                borderLeftColor: 'transparent',
                borderBottomColor: 'currentColor',
                borderRightColor: 'currentColor',
              },
              ':active:hover': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':active:focus': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':hover': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
              },
              ':focus-visible': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '&.glass:hover': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass.btn-active': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass:focus-visible': {
                outline: '2px solid 0 0 2px currentColor',
              },
              '&.loading.btn-square:before': {
                marginRight: '0px',
              },
              '&.loading.btn-circle:before': {
                marginRight: '0px',
              },
              '&.loading.btn-xl:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-lg:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-sm:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '&.loading.btn-xs:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-group > input[type="radio"]:checked&': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-group > input[type="radio"]:checked&:focus-visible': {
                outline: '2px solid hsl(var(--p))',
              },
              '.btn-group > &:not(:first-of-type)': {
                marginLeft: '-1px',
                borderTopLeftRadius: '0px',
                borderBottomLeftRadius: '0px',
              },
              '.btn-group > &:not(:last-of-type)': {
                borderTopRightRadius: '0px',
                borderBottomRightRadius: '0px',
              },
              '@media (prefers-reduced-motion: reduce)': {
                '&.loading:before': {
                  animation: 'spin 10s linear infinite',
                },
              },
            }}
            className="btn"
          />
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            display: 'grid',
            width: '100%',
            overflow: 'hidden',
            height: '100dvh',
            '&.drawer-end': {
              direction: 'rtl',
            },
            '&.drawer-end > *': {
              direction: 'ltr',
            },
            '&.drawer-end > .drawer-toggle ~ .drawer-side > .drawer-overlay + *':
              {
                '--tw-translate-x': '100%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                justifySelf: 'end',
              },
            '&.drawer-end > .drawer-toggle:checked ~ .drawer-side > .drawer-overlay + *':
              {
                '--tw-translate-x': '0px',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
            '&.drawer-end > .drawer-toggle:checked ~ .drawer-content': {
              '--tw-translate-x': '-0.5rem',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
          }}
          className="drawer"
        >
          <input
            id="my-drawer"
            type="checkbox"
            css={{
              '.drawer.drawer-end > & ~ .drawer-side > .drawer-overlay + *': {
                '--tw-translate-x': '100%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                justifySelf: 'end',
              },
              '.drawer.drawer-end > &:checked ~ .drawer-side > .drawer-overlay + *':
                {
                  '--tw-translate-x': '0px',
                  transform:
                    'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                },
              '.drawer.drawer-end > &:checked ~ .drawer-content': {
                '--tw-translate-x': '-0.5rem',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              ':where(& ~ .drawer-content)': {
                height: 'inherit',
              },
              position: 'absolute',
              height: '0px',
              width: '0px',
              appearance: 'none',
              opacity: '0',
              '& ~ .drawer-content': {
                zIndex: '0',
                gridColumnStart: '1',
                gridRowStart: '1',
                overflowY: 'auto',
                transitionProperty: 'all',
                transitionDuration: '300ms',
                transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              },
              '& ~ .drawer-side': {
                gridColumnStart: '1',
                gridRowStart: '1',
                display: 'grid',
                maxHeight: '100vh',
              },
              '& ~ .drawer-side > .drawer-overlay': {
                visibility: 'hidden',
                gridColumnStart: '1',
                gridRowStart: '1',
                opacity: '0',
                cursor: 'pointer',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
                transitionProperty: 'all',
                transitionDuration: '300ms',
                transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              },
              '& ~ .drawer-side > .drawer-overlay + *': {
                zIndex: '10',
                gridColumnStart: '1',
                gridRowStart: '1',
                '--tw-translate-x': '-100%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                transitionProperty: 'all',
                transitionDuration: '300ms',
                transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              },
              '&:checked ~ .drawer-side': {
                overflowY: 'auto',
              },
              '&:checked ~ .drawer-side > .drawer-overlay': {
                visibility: 'visible',
                opacity: '0.999999',
                '--tw-bg-opacity': '0.4',
              },
              '&:checked ~ .drawer-side > .drawer-overlay + *': {
                '--tw-translate-x': '0px',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '[dir="rtl"] & ~ .drawer-side > .drawer-overlay + *': {
                '--tw-translate-x': '100%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '[dir="rtl"] &:checked ~ .drawer-side > .drawer-overlay + *': {
                '--tw-translate-x': '0px',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '&:checked ~ .drawer-content': {
                '--tw-translate-x': '0.5rem',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '&:focus-visible ~ .drawer-content .drawer-button': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '&:focus-visible ~ .drawer-content .drawer-button.btn-primary': {
                outline: '2px solid hsl(var(--p))',
              },
              '&:focus-visible ~ .drawer-content .drawer-button.btn-secondary':
                {
                  outline: '2px solid hsl(var(--s))',
                },
              '&:focus-visible ~ .drawer-content .drawer-button.btn-accent': {
                outline: '2px solid hsl(var(--a))',
              },
              '&:focus-visible ~ .drawer-content .drawer-button.btn-info': {
                outline: '2px solid hsl(var(--in))',
              },
              '&:focus-visible ~ .drawer-content .drawer-button.btn-success': {
                outline: '2px solid hsl(var(--su))',
              },
              '&:focus-visible ~ .drawer-content .drawer-button.btn-warning': {
                outline: '2px solid hsl(var(--wa))',
              },
              '&:focus-visible ~ .drawer-content .drawer-button.btn-error': {
                outline: '2px solid hsl(var(--er))',
              },
              '&:focus-visible ~ .drawer-content .drawer-button.glass': {
                outline: '2px solid currentColor',
              },
              '&:focus-visible ~ .drawer-content .drawer-button.btn-ghost': {
                outline: '2px solid currentColor',
              },
              '&:focus-visible ~ .drawer-content .drawer-button.btn-link': {
                outline: '2px solid currentColor',
              },
              '@media (min-width: 1024px)': {
                '.drawer-mobile > & ~ .drawer-content': {
                  height: 'auto',
                },
                '.drawer-mobile > & ~ .drawer-side': {
                  overflowY: 'auto',
                },
                '.drawer-mobile.drawer-end > & ~ .drawer-content': {
                  height: 'auto',
                },
                '.drawer-mobile.drawer-end > & ~ .drawer-side': {
                  overflowY: 'auto',
                },
                '@media (min-width: 1024px)': {
                  '.drawer-mobile > & ~ .drawer-content': {
                    gridColumnStart: '2',
                  },
                  '.drawer-mobile > & ~ .drawer-side > .drawer-overlay': {
                    visibility: 'visible',
                  },
                  '.drawer-mobile > & ~ .drawer-side > .drawer-overlay + *': {
                    '--tw-translate-x': '0px',
                    transform:
                      'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                  },
                  '.drawer-mobile.drawer-end > & ~ .drawer-content': {
                    gridColumnStart: '1',
                  },
                  '.drawer-mobile.drawer-end > & ~ .drawer-side': {
                    gridColumnStart: '2',
                  },
                  '.drawer-mobile.drawer-end > & ~ .drawer-side > .drawer-overlay':
                    {
                      visibility: 'visible',
                    },
                  '.drawer-mobile.drawer-end > & ~ .drawer-side > .drawer-overlay + *':
                    {
                      '--tw-translate-x': '0px',
                      transform:
                        'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                    },
                },
                '.drawer-mobile > &:checked ~ .drawer-content': {
                  '--tw-translate-x': '0px',
                  transform:
                    'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                },
              },
            }}
          />
          <div
            css={{
              '.drawer.drawer-end > .drawer-toggle:checked ~ &': {
                '--tw-translate-x': '-0.5rem',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              ':where(.drawer-toggle ~ &)': {
                height: 'inherit',
              },
              '.drawer-toggle ~ &': {
                zIndex: '0',
                gridColumnStart: '1',
                gridRowStart: '1',
                overflowY: 'auto',
                transitionProperty: 'all',
                transitionDuration: '300ms',
                transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              },
              '.drawer-toggle:checked ~ &': {
                '--tw-translate-x': '0.5rem',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.drawer-toggle:focus-visible ~ & .drawer-button': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '.drawer-toggle:focus-visible ~ & .drawer-button.btn-primary': {
                outline: '2px solid hsl(var(--p))',
              },
              '.drawer-toggle:focus-visible ~ & .drawer-button.btn-secondary': {
                outline: '2px solid hsl(var(--s))',
              },
              '.drawer-toggle:focus-visible ~ & .drawer-button.btn-accent': {
                outline: '2px solid hsl(var(--a))',
              },
              '.drawer-toggle:focus-visible ~ & .drawer-button.btn-info': {
                outline: '2px solid hsl(var(--in))',
              },
              '.drawer-toggle:focus-visible ~ & .drawer-button.btn-success': {
                outline: '2px solid hsl(var(--su))',
              },
              '.drawer-toggle:focus-visible ~ & .drawer-button.btn-warning': {
                outline: '2px solid hsl(var(--wa))',
              },
              '.drawer-toggle:focus-visible ~ & .drawer-button.btn-error': {
                outline: '2px solid hsl(var(--er))',
              },
              '.drawer-toggle:focus-visible ~ & .drawer-button.glass': {
                outline: '2px solid currentColor',
              },
              '.drawer-toggle:focus-visible ~ & .drawer-button.btn-ghost': {
                outline: '2px solid currentColor',
              },
              '.drawer-toggle:focus-visible ~ & .drawer-button.btn-link': {
                outline: '2px solid currentColor',
              },
              '@media (min-width: 1024px)': {
                '.drawer-mobile > .drawer-toggle ~ &': {
                  height: 'auto',
                },
                '.drawer-mobile.drawer-end > .drawer-toggle ~ &': {
                  height: 'auto',
                },
                '@media (min-width: 1024px)': {
                  '.drawer-mobile > .drawer-toggle ~ &': {
                    gridColumnStart: '2',
                  },
                  '.drawer-mobile.drawer-end > .drawer-toggle ~ &': {
                    gridColumnStart: '1',
                  },
                },
                '.drawer-mobile > .drawer-toggle:checked ~ &': {
                  '--tw-translate-x': '0px',
                  transform:
                    'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                },
              },
            }}
            className="drawer-content"
          >
            <label
              htmlFor="my-drawer"
              css={{
                display: 'inline-flex',
                flexShrink: '0',
                cursor: 'pointer',
                userSelect: 'none',
                flexWrap: 'wrap',
                alignItems: 'center',
                justifyContent: 'center',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                textAlign: 'center',
                transitionProperty:
                  'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
                transitionDuration: '200ms',
                transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
                borderRadius: 'var(--rounded-btn, 0.5rem)',
                height: '3rem',
                paddingLeft: '1rem',
                paddingRight: '1rem',
                fontSize: '0.875rem',
                lineHeight: '1em',
                minHeight: '3rem',
                fontWeight: '600',
                textTransform: 'var(--btn-text-case, uppercase)',
                textDecorationLine: 'none',
                borderWidth: 'var(--border-btn, 1px)',
                animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
                '--tw-border-opacity': '1',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                '&.loading': {
                  pointerEvents: 'none',
                },
                '&.loading:hover': {
                  pointerEvents: 'none',
                },
                '&.loading:before': {
                  marginRight: '0.5rem',
                  height: '1rem',
                  width: '1rem',
                  borderRadius: '9999px',
                  borderWidth: '2px',
                  animation: 'spin 2s linear infinite',
                  content: '""',
                  borderTopColor: 'transparent',
                  borderLeftColor: 'transparent',
                  borderBottomColor: 'currentColor',
                  borderRightColor: 'currentColor',
                },
                ':active:hover': {
                  animation: 'none',
                  transform: 'scale(var(--btn-focus-scale, 0.95))',
                },
                ':active:focus': {
                  animation: 'none',
                  transform: 'scale(var(--btn-focus-scale, 0.95))',
                },
                ':hover': {
                  '--tw-border-opacity': '1',
                  borderColor:
                    'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                  '--tw-bg-opacity': '1',
                  backgroundColor:
                    'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                },
                ':focus-visible': {
                  outline: '2px solid hsl(var(--p))',
                  outlineOffset: '2px',
                },
                '&.glass:hover': {
                  '--glass-opacity': '25%',
                  '--glass-border-opacity': '15%',
                },
                '&.glass.btn-active': {
                  '--glass-opacity': '25%',
                  '--glass-border-opacity': '15%',
                },
                '&.glass:focus-visible': {
                  outline: '2px solid 0 0 2px currentColor',
                },
                '&.loading.btn-square:before': {
                  marginRight: '0px',
                },
                '&.loading.btn-circle:before': {
                  marginRight: '0px',
                },
                '&.loading.btn-xl:before': {
                  height: '1.25rem',
                  width: '1.25rem',
                },
                '&.loading.btn-lg:before': {
                  height: '1.25rem',
                  width: '1.25rem',
                },
                '&.loading.btn-sm:before': {
                  height: '0.75rem',
                  width: '0.75rem',
                },
                '&.loading.btn-xs:before': {
                  height: '0.75rem',
                  width: '0.75rem',
                },
                '.btn-group > input[type="radio"]:checked&': {
                  '--tw-border-opacity': '1',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                  '--tw-bg-opacity': '1',
                  backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                  '--tw-text-opacity': '1',
                  color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                },
                '.btn-group > input[type="radio"]:checked&:focus-visible': {
                  outline: '2px solid hsl(var(--p))',
                },
                '.btn-group > &:not(:first-of-type)': {
                  marginLeft: '-1px',
                  borderTopLeftRadius: '0px',
                  borderBottomLeftRadius: '0px',
                },
                '.btn-group > &:not(:last-of-type)': {
                  borderTopRightRadius: '0px',
                  borderBottomRightRadius: '0px',
                },
                '@media (prefers-reduced-motion: reduce)': {
                  '&.loading:before': {
                    animation: 'spin 10s linear infinite',
                  },
                },
                '.btn-outline& .badge': {
                  '--tw-border-opacity': '1',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                  '--tw-bg-opacity': '1',
                  backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                  '--tw-text-opacity': '1',
                  color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                },
                '.btn-outline&:hover .badge': {
                  '--tw-border-opacity': '1',
                  borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                  '--tw-bg-opacity': '1',
                  backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                  '--tw-text-opacity': '1',
                  color: 'hsl(var(--p) / var(--tw-text-opacity))',
                },
                '.btn-outline&:hover .badge.outline': {
                  '--tw-border-opacity': '1',
                  borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                  '--tw-bg-opacity': '1',
                  backgroundColor:
                    'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                  '--tw-text-opacity': '1',
                  color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                },
                '.drawer-toggle:focus-visible ~ .drawer-content .drawer-button&':
                  {
                    outline: '2px solid hsl(var(--p))',
                  },
                '.btn-outline& .badge-outline': {
                  '--tw-border-opacity': '1',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                  backgroundColor: 'transparent',
                  '--tw-text-opacity': '1',
                  color: 'hsl(var(--p) / var(--tw-text-opacity))',
                },
                '.btn-outline&': {
                  '--tw-text-opacity': '1',
                  color: 'hsl(var(--p) / var(--tw-text-opacity))',
                },
                '.btn-outline&:hover': {
                  '--tw-border-opacity': '1',
                  borderColor:
                    'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                  '--tw-bg-opacity': '1',
                  backgroundColor:
                    'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                  '--tw-text-opacity': '1',
                  color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                },
                '&.btn-active': {
                  '--tw-border-opacity': '1',
                  borderColor:
                    'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                  '--tw-bg-opacity': '1',
                  backgroundColor:
                    'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                },
                '.drawer-toggle:focus-visible ~ .drawer-content &': {
                  outline: '2px solid hsl(var(--nf))',
                  outlineOffset: '2px',
                },
                '.drawer-toggle:focus-visible ~ .drawer-content &.btn-primary':
                  {
                    outline: '2px solid hsl(var(--p))',
                  },
                '.drawer-toggle:focus-visible ~ .drawer-content &.btn-secondary':
                  {
                    outline: '2px solid hsl(var(--s))',
                  },
                '.drawer-toggle:focus-visible ~ .drawer-content &.btn-accent': {
                  outline: '2px solid hsl(var(--a))',
                },
                '.drawer-toggle:focus-visible ~ .drawer-content &.btn-info': {
                  outline: '2px solid hsl(var(--in))',
                },
                '.drawer-toggle:focus-visible ~ .drawer-content &.btn-success':
                  {
                    outline: '2px solid hsl(var(--su))',
                  },
                '.drawer-toggle:focus-visible ~ .drawer-content &.btn-warning':
                  {
                    outline: '2px solid hsl(var(--wa))',
                  },
                '.drawer-toggle:focus-visible ~ .drawer-content &.btn-error': {
                  outline: '2px solid hsl(var(--er))',
                },
                '.drawer-toggle:focus-visible ~ .drawer-content &.glass': {
                  outline: '2px solid currentColor',
                },
                '.drawer-toggle:focus-visible ~ .drawer-content &.btn-ghost': {
                  outline: '2px solid currentColor',
                },
                '.drawer-toggle:focus-visible ~ .drawer-content &.btn-link': {
                  outline: '2px solid currentColor',
                },
              }}
            >
              Open drawer
            </label>
          </div>
          <div
            css={{
              '.drawer.drawer-end > .drawer-toggle ~ & > .drawer-overlay + *': {
                '--tw-translate-x': '100%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                justifySelf: 'end',
              },
              '.drawer.drawer-end > .drawer-toggle:checked ~ & > .drawer-overlay + *':
                {
                  '--tw-translate-x': '0px',
                  transform:
                    'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                },
              '.drawer-toggle ~ &': {
                gridColumnStart: '1',
                gridRowStart: '1',
                display: 'grid',
                maxHeight: '100vh',
              },
              '.drawer-toggle ~ & > .drawer-overlay': {
                visibility: 'hidden',
                gridColumnStart: '1',
                gridRowStart: '1',
                opacity: '0',
                cursor: 'pointer',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
                transitionProperty: 'all',
                transitionDuration: '300ms',
                transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              },
              '.drawer-toggle ~ & > .drawer-overlay + *': {
                zIndex: '10',
                gridColumnStart: '1',
                gridRowStart: '1',
                '--tw-translate-x': '-100%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                transitionProperty: 'all',
                transitionDuration: '300ms',
                transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              },
              '.drawer-toggle:checked ~ &': {
                overflowY: 'auto',
              },
              '.drawer-toggle:checked ~ & > .drawer-overlay': {
                visibility: 'visible',
                opacity: '0.999999',
                '--tw-bg-opacity': '0.4',
              },
              '.drawer-toggle:checked ~ & > .drawer-overlay + *': {
                '--tw-translate-x': '0px',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '[dir="rtl"] .drawer-toggle ~ & > .drawer-overlay + *': {
                '--tw-translate-x': '100%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '[dir="rtl"] .drawer-toggle:checked ~ & > .drawer-overlay + *': {
                '--tw-translate-x': '0px',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '@media (min-width: 1024px)': {
                '.drawer-mobile > .drawer-toggle ~ &': {
                  overflowY: 'auto',
                },
                '.drawer-mobile.drawer-end > .drawer-toggle ~ &': {
                  overflowY: 'auto',
                },
                '@media (min-width: 1024px)': {
                  '.drawer-mobile > .drawer-toggle ~ & > .drawer-overlay': {
                    visibility: 'visible',
                  },
                  '.drawer-mobile > .drawer-toggle ~ & > .drawer-overlay + *': {
                    '--tw-translate-x': '0px',
                    transform:
                      'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                  },
                  '.drawer-mobile.drawer-end > .drawer-toggle ~ &': {
                    gridColumnStart: '2',
                  },
                  '.drawer-mobile.drawer-end > .drawer-toggle ~ & > .drawer-overlay':
                    {
                      visibility: 'visible',
                    },
                  '.drawer-mobile.drawer-end > .drawer-toggle ~ & > .drawer-overlay + *':
                    {
                      '--tw-translate-x': '0px',
                      transform:
                        'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                    },
                },
              },
            }}
            className="drawer-side"
          >
            <label
              htmlFor="my-drawer"
              css={{
                '.drawer.drawer-end > .drawer-toggle ~ .drawer-side > & + *': {
                  '--tw-translate-x': '100%',
                  transform:
                    'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                  justifySelf: 'end',
                },
                '.drawer.drawer-end > .drawer-toggle:checked ~ .drawer-side > & + *':
                  {
                    '--tw-translate-x': '0px',
                    transform:
                      'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                  },
                '.drawer-toggle ~ .drawer-side > &': {
                  visibility: 'hidden',
                  gridColumnStart: '1',
                  gridRowStart: '1',
                  opacity: '0',
                  cursor: 'pointer',
                  '--tw-bg-opacity': '1',
                  backgroundColor:
                    'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
                  transitionProperty: 'all',
                  transitionDuration: '300ms',
                  transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
                },
                '.drawer-toggle ~ .drawer-side > & + *': {
                  zIndex: '10',
                  gridColumnStart: '1',
                  gridRowStart: '1',
                  '--tw-translate-x': '-100%',
                  transform:
                    'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                  transitionProperty: 'all',
                  transitionDuration: '300ms',
                  transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
                },
                '.drawer-toggle:checked ~ .drawer-side > &': {
                  visibility: 'visible',
                  opacity: '0.999999',
                  '--tw-bg-opacity': '0.4',
                },
                '.drawer-toggle:checked ~ .drawer-side > & + *': {
                  '--tw-translate-x': '0px',
                  transform:
                    'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                },
                '[dir="rtl"] .drawer-toggle ~ .drawer-side > & + *': {
                  '--tw-translate-x': '100%',
                  transform:
                    'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                },
                '[dir="rtl"] .drawer-toggle:checked ~ .drawer-side > & + *': {
                  '--tw-translate-x': '0px',
                  transform:
                    'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                },
                '@media (min-width: 1024px)': {
                  '@media (min-width: 1024px)': {
                    '.drawer-mobile > .drawer-toggle ~ .drawer-side > &': {
                      visibility: 'visible',
                    },
                    '.drawer-mobile > .drawer-toggle ~ .drawer-side > & + *': {
                      '--tw-translate-x': '0px',
                      transform:
                        'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                    },
                    '.drawer-mobile.drawer-end > .drawer-toggle ~ .drawer-side > &':
                      {
                        visibility: 'visible',
                      },
                    '.drawer-mobile.drawer-end > .drawer-toggle ~ .drawer-side > & + *':
                      {
                        '--tw-translate-x': '0px',
                        transform:
                          'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                      },
                  },
                },
              }}
              className="drawer-overlay"
            ></label>
            <ul
              className="menu"
              css={{
                display: 'flex',
                flexDirection: 'column',
                '&.horizontal': {
                  display: 'inline-flex',
                  flexDirection: 'row',
                },
                '&.horizontal :where(li)': {
                  flexDirection: 'row',
                },
                '& :where(li)': {
                  position: 'relative',
                  display: 'flex',
                  flexDirection: 'column',
                  flexWrap: 'wrap',
                  alignItems: 'stretch',
                },
                '& :where(li:not(.menu-title)) > :where(*:not(ul))': {
                  display: 'flex',
                },
                '& :where(li:not(.disabled):not(.menu-title)) > :where(*:not(ul))':
                  {
                    cursor: 'pointer',
                    userSelect: 'none',
                    alignItems: 'center',
                    outline: '2px solid transparent',
                    outlineOffset: '2px',
                  },
                '& > :where(li > *:not(ul):focus)': {
                  outline: '2px solid transparent',
                  outlineOffset: '2px',
                },
                '& > :where(li.disabled > *:not(ul):focus)': {
                  cursor: 'auto',
                },
                '& > :where(li) :where(ul)': {
                  display: 'flex',
                  flexDirection: 'column',
                  alignItems: 'stretch',
                },
                '& > :where(li) > :where(ul)': {
                  position: 'absolute',
                  display: 'none',
                  top: 'initial',
                  left: '100%',
                  borderTopLeftRadius: 'inherit',
                  borderTopRightRadius: 'inherit',
                  borderBottomRightRadius: 'inherit',
                  borderBottomLeftRadius: 'inherit',
                },
                '& > :where(li:hover) > :where(ul)': {
                  display: 'flex',
                },
                '& > :where(li:focus) > :where(ul)': {
                  display: 'flex',
                },
                '&.horizontal li.bordered > a': {
                  borderLeftWidth: '0px',
                  borderBottomWidth: '4px',
                  '--tw-border-opacity': '1',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                },
                '&.horizontal li.bordered > button': {
                  borderLeftWidth: '0px',
                  borderBottomWidth: '4px',
                  '--tw-border-opacity': '1',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                },
                '&.horizontal li.bordered > span': {
                  borderLeftWidth: '0px',
                  borderBottomWidth: '4px',
                  '--tw-border-opacity': '1',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                },
                '& :where(li.bordered > *)': {
                  borderLeftWidth: '4px',
                  '--tw-border-opacity': '1',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                },
                '& :where(li) > :where(*:not(ul))': {
                  gap: '0.75rem',
                  paddingLeft: '1rem',
                  paddingRight: '1rem',
                  paddingTop: '0.75rem',
                  paddingBottom: '0.75rem',
                  color: 'currentColor',
                },
                '& :where(li:not(.menu-title):not(:empty)) > :where(*:not(ul):focus)':
                  {
                    backgroundColor: 'hsl(var(--bc) / var(--tw-bg-opacity))',
                    '--tw-bg-opacity': '0.1',
                  },
                '& :where(li:not(.menu-title):not(:empty)) > :where(*:not(ul):hover)':
                  {
                    backgroundColor: 'hsl(var(--bc) / var(--tw-bg-opacity))',
                    '--tw-bg-opacity': '0.1',
                  },
                '& :where(li:not(.menu-title):not(:empty)) > :where(:not(ul).active)':
                  {
                    '--tw-bg-opacity': '1',
                    backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                '& :where(li:not(.menu-title):not(:empty)) > :where(*:not(ul):active)':
                  {
                    '--tw-bg-opacity': '1',
                    backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                '& :where(li:empty)': {
                  marginLeft: '1rem',
                  marginRight: '1rem',
                  marginTop: '0.5rem',
                  marginBottom: '0.5rem',
                  height: '1px',
                  backgroundColor: 'hsl(var(--bc) / var(--tw-bg-opacity))',
                  '--tw-bg-opacity': '0.1',
                },
                '& li.disabled > *': {
                  userSelect: 'none',
                  color: 'hsl(var(--bc) / var(--tw-text-opacity))',
                  '--tw-text-opacity': '0.2',
                },
                '& li.disabled > *:hover': {
                  backgroundColor: 'transparent',
                },
                '& li.hover-bordered a': {
                  borderLeftWidth: '4px',
                  borderColor: 'transparent',
                },
                '& li.hover-bordered a:hover': {
                  '--tw-border-opacity': '1',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                },
                '&.compact li > a': {
                  paddingTop: '0.5rem',
                  paddingBottom: '0.5rem',
                  fontSize: '0.875rem',
                  lineHeight: '1.25rem',
                },
                '&.compact li > span': {
                  paddingTop: '0.5rem',
                  paddingBottom: '0.5rem',
                  fontSize: '0.875rem',
                  lineHeight: '1.25rem',
                },
                '& .menu-title > *': {
                  paddingTop: '0.25rem',
                  paddingBottom: '0.25rem',
                  fontSize: '0.75rem',
                  lineHeight: '1rem',
                  fontWeight: '700',
                  color: 'hsl(var(--bc) / var(--tw-text-opacity))',
                  '--tw-text-opacity': '0.4',
                },
                '& :where(li:not(.disabled)) > :where(*:not(ul))': {
                  outline: '2px solid transparent',
                  outlineOffset: '2px',
                  transitionProperty:
                    'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
                  transitionDuration: '200ms',
                  transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
                },
                '& > :where(li:first-child)': {
                  borderTopLeftRadius: 'inherit',
                  borderTopRightRadius: 'inherit',
                  borderBottomRightRadius: 'unset',
                  borderBottomLeftRadius: 'unset',
                },
                '& > :where(li:first-child) > :where(:not(ul))': {
                  borderTopLeftRadius: 'inherit',
                  borderTopRightRadius: 'inherit',
                  borderBottomRightRadius: 'unset',
                  borderBottomLeftRadius: 'unset',
                },
                '& > :where(li:last-child)': {
                  borderTopLeftRadius: 'unset',
                  borderTopRightRadius: 'unset',
                  borderBottomRightRadius: 'inherit',
                  borderBottomLeftRadius: 'inherit',
                },
                '& > :where(li:last-child) > :where(:not(ul))': {
                  borderTopLeftRadius: 'unset',
                  borderTopRightRadius: 'unset',
                  borderBottomRightRadius: 'inherit',
                  borderBottomLeftRadius: 'inherit',
                },
                '& > :where(li) > :where(ul) :where(li)': {
                  width: '100%',
                  whiteSpace: 'nowrap',
                },
                '& > :where(li) > :where(ul) :where(li) :where(ul)': {
                  paddingLeft: '1rem',
                },
                '& > :where(li) > :where(ul) :where(li) > :where(:not(ul))': {
                  width: '100%',
                  whiteSpace: 'nowrap',
                },
                '& > :where(li) > :where(ul) > :where(li:first-child)': {
                  borderTopLeftRadius: 'inherit',
                  borderTopRightRadius: 'inherit',
                  borderBottomRightRadius: 'unset',
                  borderBottomLeftRadius: 'unset',
                },
                '& > :where(li) > :where(ul) > :where(li:first-child) > :where(:not(ul))':
                  {
                    borderTopLeftRadius: 'inherit',
                    borderTopRightRadius: 'inherit',
                    borderBottomRightRadius: 'unset',
                    borderBottomLeftRadius: 'unset',
                  },
                '& > :where(li) > :where(ul) > :where(li:last-child)': {
                  borderTopLeftRadius: 'unset',
                  borderTopRightRadius: 'unset',
                  borderBottomRightRadius: 'inherit',
                  borderBottomLeftRadius: 'inherit',
                },
                '& > :where(li) > :where(ul) > :where(li:last-child) > :where(:not(ul))':
                  {
                    borderTopLeftRadius: 'unset',
                    borderTopRightRadius: 'unset',
                    borderBottomRightRadius: 'inherit',
                    borderBottomLeftRadius: 'inherit',
                  },
                padding: '1rem',
                overflowY: 'auto',
                width: '20rem',
                backgroundColor: 'hsl(var(--b1))',
                color: 'hsl(var(--bc))',
              }}
            >
              <li>
                <a>Sidebar Item 1</a>
              </li>
              <li>
                <a>Sidebar Item 2</a>
              </li>
            </ul>
          </div>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            display: 'grid',
            width: '100%',
            placeItems: 'center',
            backgroundSize: 'cover',
            backgroundPosition: 'center',
            '& > *': {
              gridColumnStart: '1',
              gridRowStart: '1',
            },
            minHeight: '100vh',
            backgroundColor: 'hsl(var(--b2, var(--b1)))',
          }}
        >
          <div
            css={{
              zIndex: '0',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              maxWidth: '80rem',
              gap: '1rem',
              padding: '1rem',
              textAlign: 'center',
            }}
          >
            <div
              css={{
                maxWidth: '28rem',
              }}
            >
              <h1
                css={{
                  fontSize: '3rem',
                  lineHeight: '1',
                  fontWeight: '700',
                }}
              >
                Hello there
              </h1>
              <p
                css={{
                  paddingTop: '1.5rem',
                  paddingBottom: '1.5rem',
                }}
              >
                Provident cupiditate voluptatem et in. Quaerat fugiat ut
                assumenda excepturi exercitationem quasi. In deleniti eaque aut
                repudiandae et a id nisi.
              </p>
              <button
                css={{
                  display: 'inline-flex',
                  flexShrink: '0',
                  cursor: 'pointer',
                  userSelect: 'none',
                  flexWrap: 'wrap',
                  alignItems: 'center',
                  justifyContent: 'center',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                  textAlign: 'center',
                  transitionProperty:
                    'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
                  transitionDuration: '200ms',
                  transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
                  borderRadius: 'var(--rounded-btn, 0.5rem)',
                  height: '3rem',
                  paddingLeft: '1rem',
                  paddingRight: '1rem',
                  fontSize: '0.875rem',
                  lineHeight: '1em',
                  minHeight: '3rem',
                  fontWeight: '600',
                  textTransform: 'var(--btn-text-case, uppercase)',
                  textDecorationLine: 'none',
                  borderWidth: 'var(--border-btn, 1px)',
                  animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
                  '--tw-border-opacity': '1',
                  '--tw-bg-opacity': '1',
                  backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                  '--tw-text-opacity': '1',
                  color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  '&.loading': {
                    pointerEvents: 'none',
                  },
                  '&.loading:hover': {
                    pointerEvents: 'none',
                  },
                  '&.loading:before': {
                    marginRight: '0.5rem',
                    height: '1rem',
                    width: '1rem',
                    borderRadius: '9999px',
                    borderWidth: '2px',
                    animation: 'spin 2s linear infinite',
                    content: '""',
                    borderTopColor: 'transparent',
                    borderLeftColor: 'transparent',
                    borderBottomColor: 'currentColor',
                    borderRightColor: 'currentColor',
                  },
                  ':active:hover': {
                    animation: 'none',
                    transform: 'scale(var(--btn-focus-scale, 0.95))',
                  },
                  ':active:focus': {
                    animation: 'none',
                    transform: 'scale(var(--btn-focus-scale, 0.95))',
                  },
                  ':hover': {
                    '--tw-border-opacity': '1',
                    borderColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                  },
                  ':focus-visible': {
                    outline: '2px solid hsl(var(--p))',
                    outlineOffset: '2px',
                  },
                  '&.glass:hover': {
                    '--glass-opacity': '25%',
                    '--glass-border-opacity': '15%',
                  },
                  '&.glass.btn-active': {
                    '--glass-opacity': '25%',
                    '--glass-border-opacity': '15%',
                  },
                  '&.glass:focus-visible': {
                    outline: '2px solid 0 0 2px currentColor',
                  },
                  '&.loading.btn-square:before': {
                    marginRight: '0px',
                  },
                  '&.loading.btn-circle:before': {
                    marginRight: '0px',
                  },
                  '&.loading.btn-xl:before': {
                    height: '1.25rem',
                    width: '1.25rem',
                  },
                  '&.loading.btn-lg:before': {
                    height: '1.25rem',
                    width: '1.25rem',
                  },
                  '&.loading.btn-sm:before': {
                    height: '0.75rem',
                    width: '0.75rem',
                  },
                  '&.loading.btn-xs:before': {
                    height: '0.75rem',
                    width: '0.75rem',
                  },
                  '.btn-group > input[type="radio"]:checked&': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                  '.btn-group > input[type="radio"]:checked&:focus-visible': {
                    outline: '2px solid hsl(var(--p))',
                  },
                  '.btn-group > &:not(:first-of-type)': {
                    marginLeft: '-1px',
                    borderTopLeftRadius: '0px',
                    borderBottomLeftRadius: '0px',
                  },
                  '.btn-group > &:not(:last-of-type)': {
                    borderTopRightRadius: '0px',
                    borderBottomRightRadius: '0px',
                  },
                  '@media (prefers-reduced-motion: reduce)': {
                    '&.loading:before': {
                      animation: 'spin 10s linear infinite',
                    },
                  },
                  '.btn-outline& .badge': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                  '.btn-outline&:hover .badge': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--p) / var(--tw-text-opacity))',
                  },
                  '.btn-outline&:hover .badge.outline': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                  '.drawer-toggle:focus-visible ~ .drawer-content .drawer-button&':
                    {
                      outline: '2px solid hsl(var(--p))',
                    },
                  '.btn-outline& .badge-outline': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                    backgroundColor: 'transparent',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--p) / var(--tw-text-opacity))',
                  },
                  '.btn-outline&': {
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--p) / var(--tw-text-opacity))',
                  },
                  '.btn-outline&:hover': {
                    '--tw-border-opacity': '1',
                    borderColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                  '&.btn-active': {
                    '--tw-border-opacity': '1',
                    borderColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                  },
                }}
              >
                Get Started
              </button>
            </div>
          </div>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            position: 'relative',
            display: 'inline-flex',
            width: 'max-content',
            '& :where(.indicator-item)': {
              zIndex: '1',
              position: 'absolute',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              right: '0px',
              left: 'auto',
              top: '0px',
              bottom: 'auto',
              '--tw-translate-x': '50%',
              '--tw-translate-y': '-50%',
            },
            '& :where(.indicator-item.indicator-start)': {
              right: 'auto',
              left: '0px',
              '--tw-translate-x': '-50%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
            '& :where(.indicator-item.indicator-center)': {
              right: '50%',
              left: '50%',
              '--tw-translate-x': '-50%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
            '& :where(.indicator-item.indicator-end)': {
              right: '0px',
              left: 'auto',
              '--tw-translate-x': '50%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
            '& :where(.indicator-item.indicator-bottom)': {
              top: 'auto',
              bottom: '0px',
              '--tw-translate-y': '50%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
            '& :where(.indicator-item.indicator-middle)': {
              top: '50%',
              bottom: '50%',
              '--tw-translate-y': '-50%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
            '& :where(.indicator-item.indicator-top)': {
              top: '0px',
              bottom: 'auto',
              '--tw-translate-y': '-50%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
          }}
          className="indicator"
        >
          <span
            css={{
              '.indicator ': {
                zIndex: '1',
                position: 'absolute',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                top: '0px',
                bottom: 'auto',
                '--tw-translate-x': '50%',
                '--tw-translate-y': '-50%',
              },
              '.indicator :where(&.indicator-start)': {
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-center)': {
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-end)': {
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-bottom)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-middle)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-top)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              display: 'inline-flex',
              alignItems: 'center',
              justifyContent: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              height: '1.25rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
              width: 'fit-content',
              paddingLeft: '0.563rem',
              paddingRight: '0.563rem',
              borderWidth: '1px',
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              borderRadius: 'var(--rounded-badge, 1.9rem)',
              '.btn-outline &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.btn-outline &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                backgroundColor: 'transparent',
              },
              '.btn-outline:hover &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              '.btn-outline:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--p) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--a) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.badge-outline&': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
            }}
            className="indicator-item"
          >
            99+
          </span>
          <button
            css={{
              display: 'inline-flex',
              flexShrink: '0',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              borderColor: 'hsl(var(--n) / var(--tw-border-opacity))',
              textAlign: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              borderRadius: 'var(--rounded-btn, 0.5rem)',
              height: '3rem',
              paddingLeft: '1rem',
              paddingRight: '1rem',
              fontSize: '0.875rem',
              lineHeight: '1em',
              minHeight: '3rem',
              fontWeight: '600',
              textTransform: 'var(--btn-text-case, uppercase)',
              textDecorationLine: 'none',
              borderWidth: 'var(--border-btn, 1px)',
              animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              '&.loading': {
                pointerEvents: 'none',
              },
              '&.loading:hover': {
                pointerEvents: 'none',
              },
              '&.loading:before': {
                marginRight: '0.5rem',
                height: '1rem',
                width: '1rem',
                borderRadius: '9999px',
                borderWidth: '2px',
                animation: 'spin 2s linear infinite',
                content: '""',
                borderTopColor: 'transparent',
                borderLeftColor: 'transparent',
                borderBottomColor: 'currentColor',
                borderRightColor: 'currentColor',
              },
              ':active:hover': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':active:focus': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':hover': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
              },
              ':focus-visible': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '&.glass:hover': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass.btn-active': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass:focus-visible': {
                outline: '2px solid 0 0 2px currentColor',
              },
              '&.loading.btn-square:before': {
                marginRight: '0px',
              },
              '&.loading.btn-circle:before': {
                marginRight: '0px',
              },
              '&.loading.btn-xl:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-lg:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-sm:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '&.loading.btn-xs:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-group > input[type="radio"]:checked&': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-group > input[type="radio"]:checked&:focus-visible': {
                outline: '2px solid hsl(var(--p))',
              },
              '.btn-group > &:not(:first-of-type)': {
                marginLeft: '-1px',
                borderTopLeftRadius: '0px',
                borderBottomLeftRadius: '0px',
              },
              '.btn-group > &:not(:last-of-type)': {
                borderTopRightRadius: '0px',
                borderBottomRightRadius: '0px',
              },
              '@media (prefers-reduced-motion: reduce)': {
                '&.loading:before': {
                  animation: 'spin 10s linear infinite',
                },
              },
            }}
          >
            inbox
          </button>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            fontSize: '0.875rem',
            lineHeight: '1.25rem',
            maxWidth: '100%',
            overflowX: 'auto',
            paddingTop: '0.5rem',
            paddingBottom: '0.5rem',
            '& > ul': {
              display: 'flex',
              alignItems: 'center',
              whiteSpace: 'nowrap',
              minHeight: 'min-content',
            },
            '& > ul > li': {
              display: 'flex',
              alignItems: 'center',
            },
            '& > ul > li > a': {
              display: 'flex',
              cursor: 'pointer',
              alignItems: 'center',
            },
            '& > ul > li > a:hover': {
              textDecorationLine: 'underline',
            },
            '& > ul > li > a:focus': {
              outline: '2px solid transparent',
              outlineOffset: '2px',
            },
            '& > ul > li > a:focus-visible': {
              outline: '2px solid currentColor',
              outlineOffset: '2px',
            },
            '& > ul > li + *:before': {
              content: '""',
              marginLeft: '0.5rem',
              marginRight: '0.75rem',
              display: 'block',
              height: '0.375rem',
              width: '0.375rem',
              '--tw-rotate': '45deg',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              opacity: '0.4',
              borderTop: '1px solid',
              borderRight: '1px solid',
              backgroundColor: 'transparent',
            },
          }}
        >
          <ul>
            <li>
              <a>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  css={{
                    width: '1rem',
                    height: '1rem',
                    marginRight: '0.5rem',
                    stroke: 'currentColor',
                  }}
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
                  ></path>
                </svg>
                Home
              </a>
            </li>
            <li>
              <a>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  css={{
                    width: '1rem',
                    height: '1rem',
                    marginRight: '0.5rem',
                    stroke: 'currentColor',
                  }}
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
                  ></path>
                </svg>
                Documents
              </a>
            </li>
            <li>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                css={{
                  width: '1rem',
                  height: '1rem',
                  marginRight: '0.5rem',
                  stroke: 'currentColor',
                }}
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  d="M9 13h6m-3-3v6m5 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                ></path>
              </svg>
              Add Document
            </li>
          </ul>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <ul
          css={{
            display: 'inline-grid',
            gridAutoFlow: 'column',
            overflow: 'hidden',
            overflowX: 'auto',
            counterReset: 'step',
            gridAutoColumns: '1fr',
            '& .step': {
              display: 'grid',
              gridTemplateColumns: 'auto',
              gridTemplateRows: '40px 1fr',
              placeItems: 'center',
              textAlign: 'center',
              minWidth: '4rem',
            },
            '& .step:before': {
              top: '0px',
              gridColumnStart: '1',
              gridRowStart: '1',
              height: '0.5rem',
              width: '100%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--b3, var(--b2)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              content: '""',
              marginLeft: '-100%',
            },
            '& .step:after': {
              content: 'counter(step)',
              counterIncrement: 'step',
              zIndex: '1',
              position: 'relative',
              gridColumnStart: '1',
              gridRowStart: '1',
              display: 'grid',
              height: '2rem',
              width: '2rem',
              placeItems: 'center',
              placeSelf: 'center',
              borderRadius: '9999px',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--b3, var(--b2)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--bc) / var(--tw-text-opacity))',
            },
            '& .step:first-child:before': {
              content: 'none',
            },
            '& .step[data-content]:after': {
              content: 'attr(data-content)',
            },
            '& .step-neutral + .step-neutral:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '& .step-neutral:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '& .step-primary + .step-primary:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '& .step-primary:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '& .step-secondary + .step-secondary:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '& .step-secondary:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '& .step-accent + .step-accent:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '& .step-accent:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '& .step-info + .step-info:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
            },
            '& .step-info:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--inc, var(--nc)) / var(--tw-text-opacity))',
            },
            '& .step-success + .step-success:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--su) / var(--tw-bg-opacity))',
            },
            '& .step-success:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--su) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--suc, var(--nc)) / var(--tw-text-opacity))',
            },
            '& .step-warning + .step-warning:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--wa) / var(--tw-bg-opacity))',
            },
            '& .step-warning:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--wa) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wac, var(--nc)) / var(--tw-text-opacity))',
            },
            '& .step-error + .step-error:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--er) / var(--tw-bg-opacity))',
            },
            '& .step-error:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--er) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--erc, var(--nc)) / var(--tw-text-opacity))',
            },
          }}
          className="steps"
        >
          <li
            className="step step-primary"
            css={{
              '.steps & + &:before': {
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.steps &:after': {
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
            }}
          >
            Register
          </li>
          <li
            className="step step-primary"
            css={{
              '.steps & + &:before': {
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.steps &:after': {
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
            }}
          >
            Choose plan
          </li>
          <li
            className="step  step-primary"
            css={{
              '.steps & + &:before': {
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.steps &:after': {
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
            }}
          >
            Purchase
          </li>
          <li className="step" css={{}}>
            Receive Product
          </li>
        </ul>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <ul
          css={{
            display: 'inline-grid',
            gridAutoFlow: 'column',
            overflow: 'hidden',
            overflowX: 'auto',
            counterReset: 'step',
            gridAutoColumns: '1fr',
            '& .step': {
              display: 'grid',
              gridTemplateColumns: 'auto',
              gridTemplateRows: '40px 1fr',
              placeItems: 'center',
              textAlign: 'center',
              minWidth: '4rem',
            },
            '& .step:before': {
              top: '0px',
              gridColumnStart: '1',
              gridRowStart: '1',
              height: '0.5rem',
              width: '100%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--b3, var(--b2)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              content: '""',
              marginLeft: '-100%',
            },
            '& .step:after': {
              content: 'counter(step)',
              counterIncrement: 'step',
              zIndex: '1',
              position: 'relative',
              gridColumnStart: '1',
              gridRowStart: '1',
              display: 'grid',
              height: '2rem',
              width: '2rem',
              placeItems: 'center',
              placeSelf: 'center',
              borderRadius: '9999px',
              '--tw-bg-opacity': '1',
              backgroundColor:
                'hsl(var(--b3, var(--b2)) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--bc) / var(--tw-text-opacity))',
            },
            '& .step:first-child:before': {
              content: 'none',
            },
            '& .step[data-content]:after': {
              content: 'attr(data-content)',
            },
            '& .step-neutral + .step-neutral:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '& .step-neutral:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            '& .step-primary + .step-primary:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '& .step-primary:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--pc) / var(--tw-text-opacity))',
            },
            '& .step-secondary + .step-secondary:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '& .step-secondary:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
            },
            '& .step-accent + .step-accent:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '& .step-accent:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--ac) / var(--tw-text-opacity))',
            },
            '& .step-info + .step-info:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
            },
            '& .step-info:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--inc, var(--nc)) / var(--tw-text-opacity))',
            },
            '& .step-success + .step-success:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--su) / var(--tw-bg-opacity))',
            },
            '& .step-success:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--su) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--suc, var(--nc)) / var(--tw-text-opacity))',
            },
            '& .step-warning + .step-warning:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--wa) / var(--tw-bg-opacity))',
            },
            '& .step-warning:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--wa) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--wac, var(--nc)) / var(--tw-text-opacity))',
            },
            '& .step-error + .step-error:before': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--er) / var(--tw-bg-opacity))',
            },
            '& .step-error:after': {
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--er) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--erc, var(--nc)) / var(--tw-text-opacity))',
            },
          }}
          className="steps"
        >
          <li
            className="step step-info"
            css={{
              '.steps &': {
                display: 'grid',
                gridTemplateColumns: 'auto',
                gridTemplateRows: '40px 1fr',
                placeItems: 'center',
                textAlign: 'center',
                minWidth: '4rem',
              },
              '.steps &:before': {
                top: '0px',
                gridColumnStart: '1',
                gridRowStart: '1',
                height: '0.5rem',
                width: '100%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b3, var(--b2)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
                content: '""',
                marginLeft: '-100%',
              },
              '.steps &:after': {
                content: 'counter(step)',
                counterIncrement: 'step',
                zIndex: '1',
                position: 'relative',
                gridColumnStart: '1',
                gridRowStart: '1',
                display: 'grid',
                height: '2rem',
                width: '2rem',
                placeItems: 'center',
                placeSelf: 'center',
                borderRadius: '9999px',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--inc, var(--nc)) / var(--tw-text-opacity))',
              },
              '.steps &:first-child:before': {
                content: 'none',
              },
              '.steps-horizontal &': {
                display: 'grid',
                gridTemplateColumns: 'auto',
                gridTemplateRows: '40px 1fr',
                placeItems: 'center',
                textAlign: 'center',
                minWidth: '4rem',
              },
              '.steps-vertical &': {
                display: 'grid',
                gridTemplateColumns: '40px 1fr',
                gridTemplateRows: 'auto',
                gap: '0.5rem',
                minHeight: '4rem',
                justifyItems: 'start',
              },
              '.steps-horizontal &:before': {
                height: '0.5rem',
                width: '100%',
                '--tw-translate-y': '0px',
                '--tw-translate-x': '0px',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                content: '""',
                marginLeft: '-100%',
              },
              '.steps-vertical &:before': {
                height: '100%',
                width: '0.5rem',
                '--tw-translate-y': '-50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                marginLeft: '50%',
              },
              '.steps & + &:before': {
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
              },
            }}
          >
            Fly to moon
          </li>
          <li
            className="step step-info"
            css={{
              '.steps &': {
                display: 'grid',
                gridTemplateColumns: 'auto',
                gridTemplateRows: '40px 1fr',
                placeItems: 'center',
                textAlign: 'center',
                minWidth: '4rem',
              },
              '.steps &:before': {
                top: '0px',
                gridColumnStart: '1',
                gridRowStart: '1',
                height: '0.5rem',
                width: '100%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b3, var(--b2)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
                content: '""',
                marginLeft: '-100%',
              },
              '.steps &:after': {
                content: 'counter(step)',
                counterIncrement: 'step',
                zIndex: '1',
                position: 'relative',
                gridColumnStart: '1',
                gridRowStart: '1',
                display: 'grid',
                height: '2rem',
                width: '2rem',
                placeItems: 'center',
                placeSelf: 'center',
                borderRadius: '9999px',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--inc, var(--nc)) / var(--tw-text-opacity))',
              },
              '.steps &:first-child:before': {
                content: 'none',
              },
              '.steps-horizontal &': {
                display: 'grid',
                gridTemplateColumns: 'auto',
                gridTemplateRows: '40px 1fr',
                placeItems: 'center',
                textAlign: 'center',
                minWidth: '4rem',
              },
              '.steps-vertical &': {
                display: 'grid',
                gridTemplateColumns: '40px 1fr',
                gridTemplateRows: 'auto',
                gap: '0.5rem',
                minHeight: '4rem',
                justifyItems: 'start',
              },
              '.steps-horizontal &:before': {
                height: '0.5rem',
                width: '100%',
                '--tw-translate-y': '0px',
                '--tw-translate-x': '0px',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                content: '""',
                marginLeft: '-100%',
              },
              '.steps-vertical &:before': {
                height: '100%',
                width: '0.5rem',
                '--tw-translate-y': '-50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                marginLeft: '50%',
              },
              '.steps & + &:before': {
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
              },
            }}
          >
            Shrink the moon
          </li>
          <li
            className="step step-info"
            css={{
              '.steps &': {
                display: 'grid',
                gridTemplateColumns: 'auto',
                gridTemplateRows: '40px 1fr',
                placeItems: 'center',
                textAlign: 'center',
                minWidth: '4rem',
              },
              '.steps &:before': {
                top: '0px',
                gridColumnStart: '1',
                gridRowStart: '1',
                height: '0.5rem',
                width: '100%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b3, var(--b2)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
                content: '""',
                marginLeft: '-100%',
              },
              '.steps &:after': {
                content: 'counter(step)',
                counterIncrement: 'step',
                zIndex: '1',
                position: 'relative',
                gridColumnStart: '1',
                gridRowStart: '1',
                display: 'grid',
                height: '2rem',
                width: '2rem',
                placeItems: 'center',
                placeSelf: 'center',
                borderRadius: '9999px',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--inc, var(--nc)) / var(--tw-text-opacity))',
              },
              '.steps &:first-child:before': {
                content: 'none',
              },
              '.steps-horizontal &': {
                display: 'grid',
                gridTemplateColumns: 'auto',
                gridTemplateRows: '40px 1fr',
                placeItems: 'center',
                textAlign: 'center',
                minWidth: '4rem',
              },
              '.steps-vertical &': {
                display: 'grid',
                gridTemplateColumns: '40px 1fr',
                gridTemplateRows: 'auto',
                gap: '0.5rem',
                minHeight: '4rem',
                justifyItems: 'start',
              },
              '.steps-horizontal &:before': {
                height: '0.5rem',
                width: '100%',
                '--tw-translate-y': '0px',
                '--tw-translate-x': '0px',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                content: '""',
                marginLeft: '-100%',
              },
              '.steps-vertical &:before': {
                height: '100%',
                width: '0.5rem',
                '--tw-translate-y': '-50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                marginLeft: '50%',
              },
              '.steps & + &:before': {
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--in) / var(--tw-bg-opacity))',
              },
            }}
          >
            Grab the moon
          </li>
          <li
            className="step step-error"
            css={{
              '.steps &': {
                display: 'grid',
                gridTemplateColumns: 'auto',
                gridTemplateRows: '40px 1fr',
                placeItems: 'center',
                textAlign: 'center',
                minWidth: '4rem',
              },
              '.steps &:before': {
                top: '0px',
                gridColumnStart: '1',
                gridRowStart: '1',
                height: '0.5rem',
                width: '100%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b3, var(--b2)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
                content: '""',
                marginLeft: '-100%',
              },
              '.steps &:after': {
                content: 'counter(step)',
                counterIncrement: 'step',
                zIndex: '1',
                position: 'relative',
                gridColumnStart: '1',
                gridRowStart: '1',
                display: 'grid',
                height: '2rem',
                width: '2rem',
                placeItems: 'center',
                placeSelf: 'center',
                borderRadius: '9999px',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--er) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--erc, var(--nc)) / var(--tw-text-opacity))',
              },
              '.steps &:first-child:before': {
                content: 'none',
              },
              '.steps-horizontal &': {
                display: 'grid',
                gridTemplateColumns: 'auto',
                gridTemplateRows: '40px 1fr',
                placeItems: 'center',
                textAlign: 'center',
                minWidth: '4rem',
              },
              '.steps-vertical &': {
                display: 'grid',
                gridTemplateColumns: '40px 1fr',
                gridTemplateRows: 'auto',
                gap: '0.5rem',
                minHeight: '4rem',
                justifyItems: 'start',
              },
              '.steps-horizontal &:before': {
                height: '0.5rem',
                width: '100%',
                '--tw-translate-y': '0px',
                '--tw-translate-x': '0px',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                content: '""',
                marginLeft: '-100%',
              },
              '.steps-vertical &:before': {
                height: '100%',
                width: '0.5rem',
                '--tw-translate-y': '-50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                marginLeft: '50%',
              },
              '.steps & + &:before': {
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--er) / var(--tw-bg-opacity))',
              },
            }}
            data-content="?"
          >
            Sit on toilet
          </li>
        </ul>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            display: 'flex',
            flexWrap: 'wrap',
            alignItems: 'flex-end',
          }}
        >
          <a
            css={{
              position: 'relative',
              display: 'inline-flex',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              textAlign: 'center',
              height: '3rem',
              fontSize: '1.125rem',
              lineHeight: '2',
              '--tab-padding': '1.25rem',
              '--tw-text-opacity': '0.5',
              '--tab-color': 'hsla(var(--bc) / var(--tw-text-opacity, 1))',
              '--tab-bg': 'hsla(var(--b1) / var(--tw-bg-opacity, 1))',
              '--tab-border-color': 'hsla(var(--b3) / var(--tw-bg-opacity, 1))',
              color: 'var(--tab-color)',
              paddingLeft: 'var(--tab-padding, 1rem)',
              paddingRight: 'var(--tab-padding, 1rem)',
              ':hover': {
                '--tw-text-opacity': '1',
              },
              '&.tab-active': {
                borderColor: 'hsl(var(--bc) / var(--tw-border-opacity))',
                '--tw-border-opacity': '1',
                '--tw-text-opacity': '1',
                backgroundColor: 'var(--tab-bg)',
                borderWidth:
                  'var(--tab-border, 1px) var(--tab-border, 1px) 0 var(--tab-border, 1px)',
                borderLeftColor: 'var(--tab-border-color)',
                borderRightColor: 'var(--tab-border-color)',
                borderTopColor: 'var(--tab-border-color)',
                paddingLeft:
                  'calc(var(--tab-padding, 1rem) - var(--tab-border, 1px))',
                paddingRight:
                  'calc(var(--tab-padding, 1rem) - var(--tab-border, 1px))',
                paddingBottom: 'var(--tab-border, 1px)',
                paddingTop: '0',
              },
              ':focus': {
                outline: '2px solid transparent',
                outlineOffset: '2px',
              },
              ':focus-visible': {
                outline: '2px solid currentColor',
                outlineOffset: '-3px',
              },
              '&:focus-visible.tab-lifted': {
                borderBottomRightRadius: 'var(--tab-radius, 0.5rem)',
                borderBottomLeftRadius: 'var(--tab-radius, 0.5rem)',
              },
              '.tab:focus-visible&': {
                borderBottomRightRadius: 'var(--tab-radius, 0.5rem)',
                borderBottomLeftRadius: 'var(--tab-radius, 0.5rem)',
              },
              '&.tab-active:before': {
                zIndex: '1',
                content: '""',
                display: 'block',
                position: 'absolute',
                width: 'var(--tab-radius, 0.5rem)',
                height: 'var(--tab-radius, 0.5rem)',
                bottom: '0',
                '--tab-grad': 'calc(68% - var(--tab-border, 1px))',
                '--tab-corner-bg':
                  'radial-gradient(circle at var(--circle-pos), transparent var(--tab-grad), var(--tab-border-color) calc(var(--tab-grad) + 0.3px), var(--tab-border-color) calc(var(--tab-grad) + var(--tab-border, 1px)), var(--tab-bg) calc(var(--tab-grad) + var(--tab-border, 1px) + 0.3px))',
                left: 'calc(var(--tab-radius, 0.5rem) * -1)',
                '--circle-pos': 'top left',
                backgroundImage: 'var(--tab-corner-bg)',
              },
              '&.tab-active:after': {
                zIndex: '1',
                content: '""',
                display: 'block',
                position: 'absolute',
                width: 'var(--tab-radius, 0.5rem)',
                height: 'var(--tab-radius, 0.5rem)',
                bottom: '0',
                '--tab-grad': 'calc(68% - var(--tab-border, 1px))',
                '--tab-corner-bg':
                  'radial-gradient(circle at var(--circle-pos), transparent var(--tab-grad), var(--tab-border-color) calc(var(--tab-grad) + 0.3px), var(--tab-border-color) calc(var(--tab-grad) + var(--tab-border, 1px)), var(--tab-bg) calc(var(--tab-grad) + var(--tab-border, 1px) + 0.3px))',
                right: 'calc(var(--tab-radius, 0.5rem) * -1)',
                '--circle-pos': 'top right',
                backgroundImage: 'var(--tab-corner-bg)',
              },
              '[dir="rtl"] &.tab-active:before': {
                '--circle-pos': 'top right',
              },
              '[dir="rtl"] &.tab-active:after': {
                '--circle-pos': 'top left',
              },
              '&.tab-active:first-child:before': {
                background: 'none',
              },
              '&.tab-active:last-child:after': {
                background: 'none',
              },
              '&.tab-active + &.tab-active:before': {
                background: 'none',
              },
              border: 'var(--tab-border, 1px) solid transparent',
              borderWidth: '0 0 var(--tab-border, 1px) 0',
              borderTopLeftRadius: 'var(--tab-radius, 0.5rem)',
              borderTopRightRadius: 'var(--tab-radius, 0.5rem)',
              borderBottomColor: 'var(--tab-border-color)',
              paddingTop: 'var(--tab-border, 1px)',
            }}
          >
            Tab 1
          </a>
          <a
            css={{
              position: 'relative',
              display: 'inline-flex',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              textAlign: 'center',
              height: '3rem',
              fontSize: '1.125rem',
              lineHeight: '2',
              '--tab-padding': '1.25rem',
              '--tw-text-opacity': '0.5',
              '--tab-color': 'hsla(var(--bc) / var(--tw-text-opacity, 1))',
              '--tab-bg': 'hsla(var(--b1) / var(--tw-bg-opacity, 1))',
              '--tab-border-color': 'hsla(var(--b3) / var(--tw-bg-opacity, 1))',
              color: 'var(--tab-color)',
              paddingLeft: 'var(--tab-padding, 1rem)',
              paddingRight: 'var(--tab-padding, 1rem)',
              ':hover': {
                '--tw-text-opacity': '1',
              },
              '&.tab-active': {
                borderColor: 'hsl(var(--bc) / var(--tw-border-opacity))',
                '--tw-border-opacity': '1',
                '--tw-text-opacity': '1',
                backgroundColor: 'var(--tab-bg)',
                borderWidth:
                  'var(--tab-border, 1px) var(--tab-border, 1px) 0 var(--tab-border, 1px)',
                borderLeftColor: 'var(--tab-border-color)',
                borderRightColor: 'var(--tab-border-color)',
                borderTopColor: 'var(--tab-border-color)',
                paddingLeft:
                  'calc(var(--tab-padding, 1rem) - var(--tab-border, 1px))',
                paddingRight:
                  'calc(var(--tab-padding, 1rem) - var(--tab-border, 1px))',
                paddingBottom: 'var(--tab-border, 1px)',
                paddingTop: '0',
              },
              ':focus': {
                outline: '2px solid transparent',
                outlineOffset: '2px',
              },
              ':focus-visible': {
                outline: '2px solid currentColor',
                outlineOffset: '-3px',
              },
              '&:focus-visible.tab-lifted': {
                borderBottomRightRadius: 'var(--tab-radius, 0.5rem)',
                borderBottomLeftRadius: 'var(--tab-radius, 0.5rem)',
              },
              '.tab:focus-visible&': {
                borderBottomRightRadius: 'var(--tab-radius, 0.5rem)',
                borderBottomLeftRadius: 'var(--tab-radius, 0.5rem)',
              },
              '&.tab-active:before': {
                zIndex: '1',
                content: '""',
                display: 'block',
                position: 'absolute',
                width: 'var(--tab-radius, 0.5rem)',
                height: 'var(--tab-radius, 0.5rem)',
                bottom: '0',
                '--tab-grad': 'calc(68% - var(--tab-border, 1px))',
                '--tab-corner-bg':
                  'radial-gradient(circle at var(--circle-pos), transparent var(--tab-grad), var(--tab-border-color) calc(var(--tab-grad) + 0.3px), var(--tab-border-color) calc(var(--tab-grad) + var(--tab-border, 1px)), var(--tab-bg) calc(var(--tab-grad) + var(--tab-border, 1px) + 0.3px))',
                left: 'calc(var(--tab-radius, 0.5rem) * -1)',
                '--circle-pos': 'top left',
                backgroundImage: 'var(--tab-corner-bg)',
              },
              '&.tab-active:after': {
                zIndex: '1',
                content: '""',
                display: 'block',
                position: 'absolute',
                width: 'var(--tab-radius, 0.5rem)',
                height: 'var(--tab-radius, 0.5rem)',
                bottom: '0',
                '--tab-grad': 'calc(68% - var(--tab-border, 1px))',
                '--tab-corner-bg':
                  'radial-gradient(circle at var(--circle-pos), transparent var(--tab-grad), var(--tab-border-color) calc(var(--tab-grad) + 0.3px), var(--tab-border-color) calc(var(--tab-grad) + var(--tab-border, 1px)), var(--tab-bg) calc(var(--tab-grad) + var(--tab-border, 1px) + 0.3px))',
                right: 'calc(var(--tab-radius, 0.5rem) * -1)',
                '--circle-pos': 'top right',
                backgroundImage: 'var(--tab-corner-bg)',
              },
              '[dir="rtl"] &.tab-active:before': {
                '--circle-pos': 'top right',
              },
              '[dir="rtl"] &.tab-active:after': {
                '--circle-pos': 'top left',
              },
              '&.tab-active:first-child:before': {
                background: 'none',
              },
              '&.tab-active:last-child:after': {
                background: 'none',
              },
              '&.tab-active + &.tab-active:before': {
                background: 'none',
              },
              border: 'var(--tab-border, 1px) solid transparent',
              borderWidth: '0 0 var(--tab-border, 1px) 0',
              borderTopLeftRadius: 'var(--tab-radius, 0.5rem)',
              borderTopRightRadius: 'var(--tab-radius, 0.5rem)',
              borderBottomColor: 'var(--tab-border-color)',
              paddingTop: 'var(--tab-border, 1px)',
            }}
            className=" tab-active"
          >
            Tab 2
          </a>
          <a
            css={{
              position: 'relative',
              display: 'inline-flex',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              textAlign: 'center',
              height: '3rem',
              fontSize: '1.125rem',
              lineHeight: '2',
              '--tab-padding': '1.25rem',
              '--tw-text-opacity': '0.5',
              '--tab-color': 'hsla(var(--bc) / var(--tw-text-opacity, 1))',
              '--tab-bg': 'hsla(var(--b1) / var(--tw-bg-opacity, 1))',
              '--tab-border-color': 'hsla(var(--b3) / var(--tw-bg-opacity, 1))',
              color: 'var(--tab-color)',
              paddingLeft: 'var(--tab-padding, 1rem)',
              paddingRight: 'var(--tab-padding, 1rem)',
              ':hover': {
                '--tw-text-opacity': '1',
              },
              '&.tab-active': {
                borderColor: 'hsl(var(--bc) / var(--tw-border-opacity))',
                '--tw-border-opacity': '1',
                '--tw-text-opacity': '1',
                backgroundColor: 'var(--tab-bg)',
                borderWidth:
                  'var(--tab-border, 1px) var(--tab-border, 1px) 0 var(--tab-border, 1px)',
                borderLeftColor: 'var(--tab-border-color)',
                borderRightColor: 'var(--tab-border-color)',
                borderTopColor: 'var(--tab-border-color)',
                paddingLeft:
                  'calc(var(--tab-padding, 1rem) - var(--tab-border, 1px))',
                paddingRight:
                  'calc(var(--tab-padding, 1rem) - var(--tab-border, 1px))',
                paddingBottom: 'var(--tab-border, 1px)',
                paddingTop: '0',
              },
              ':focus': {
                outline: '2px solid transparent',
                outlineOffset: '2px',
              },
              ':focus-visible': {
                outline: '2px solid currentColor',
                outlineOffset: '-3px',
              },
              '&:focus-visible.tab-lifted': {
                borderBottomRightRadius: 'var(--tab-radius, 0.5rem)',
                borderBottomLeftRadius: 'var(--tab-radius, 0.5rem)',
              },
              '.tab:focus-visible&': {
                borderBottomRightRadius: 'var(--tab-radius, 0.5rem)',
                borderBottomLeftRadius: 'var(--tab-radius, 0.5rem)',
              },
              '&.tab-active:before': {
                zIndex: '1',
                content: '""',
                display: 'block',
                position: 'absolute',
                width: 'var(--tab-radius, 0.5rem)',
                height: 'var(--tab-radius, 0.5rem)',
                bottom: '0',
                '--tab-grad': 'calc(68% - var(--tab-border, 1px))',
                '--tab-corner-bg':
                  'radial-gradient(circle at var(--circle-pos), transparent var(--tab-grad), var(--tab-border-color) calc(var(--tab-grad) + 0.3px), var(--tab-border-color) calc(var(--tab-grad) + var(--tab-border, 1px)), var(--tab-bg) calc(var(--tab-grad) + var(--tab-border, 1px) + 0.3px))',
                left: 'calc(var(--tab-radius, 0.5rem) * -1)',
                '--circle-pos': 'top left',
                backgroundImage: 'var(--tab-corner-bg)',
              },
              '&.tab-active:after': {
                zIndex: '1',
                content: '""',
                display: 'block',
                position: 'absolute',
                width: 'var(--tab-radius, 0.5rem)',
                height: 'var(--tab-radius, 0.5rem)',
                bottom: '0',
                '--tab-grad': 'calc(68% - var(--tab-border, 1px))',
                '--tab-corner-bg':
                  'radial-gradient(circle at var(--circle-pos), transparent var(--tab-grad), var(--tab-border-color) calc(var(--tab-grad) + 0.3px), var(--tab-border-color) calc(var(--tab-grad) + var(--tab-border, 1px)), var(--tab-bg) calc(var(--tab-grad) + var(--tab-border, 1px) + 0.3px))',
                right: 'calc(var(--tab-radius, 0.5rem) * -1)',
                '--circle-pos': 'top right',
                backgroundImage: 'var(--tab-corner-bg)',
              },
              '[dir="rtl"] &.tab-active:before': {
                '--circle-pos': 'top right',
              },
              '[dir="rtl"] &.tab-active:after': {
                '--circle-pos': 'top left',
              },
              '&.tab-active:first-child:before': {
                background: 'none',
              },
              '&.tab-active:last-child:after': {
                background: 'none',
              },
              '&.tab-active + &.tab-active:before': {
                background: 'none',
              },
              border: 'var(--tab-border, 1px) solid transparent',
              borderWidth: '0 0 var(--tab-border, 1px) 0',
              borderTopLeftRadius: 'var(--tab-radius, 0.5rem)',
              borderTopRightRadius: 'var(--tab-radius, 0.5rem)',
              borderBottomColor: 'var(--tab-border-color)',
              paddingTop: 'var(--tab-border, 1px)',
            }}
          >
            Tab 3
          </a>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            position: 'relative',
            overflow: 'hidden',
            overflowX: 'auto',
            minWidth: '18rem',
            '--tw-bg-opacity': '1',
            backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
            paddingTop: '1.25rem',
            paddingBottom: '1.25rem',
            '--tw-text-opacity': '1',
            color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            borderRadius: 'var(--rounded-box, 1rem)',
            '& pre[data-prefix]:before': {
              content: 'attr(data-prefix)',
              display: 'inline-block',
              textAlign: 'right',
              width: '2rem',
              opacity: '0.5',
            },
            ':before': {
              content: '""',
              marginBottom: '1rem',
              display: 'block',
              height: '0.75rem',
              width: '0.75rem',
              borderRadius: '9999px',
              opacity: '0.3',
              boxShadow: '1.4em 0, 2.8em 0, 4.2em 0',
            },
            '& pre': {
              paddingRight: '1.25rem',
            },
            '& pre:before': {
              content: '""',
              marginRight: '2ch',
            },
          }}
        >
          <pre data-prefix="$">
            <code>npm i daisyui</code>
          </pre>
          <pre
            data-prefix=">"
            css={{
              color: 'hsl(var(--wa))',
            }}
          >
            <code>installing...</code>
          </pre>
          <pre
            data-prefix=">"
            css={{
              color: 'hsl(var(--su))',
            }}
          >
            <code>Done!</code>
          </pre>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            position: 'relative',
            display: 'flex',
            flexDirection: 'column',
            overflow: 'hidden',
            borderRadius: 'var(--rounded-box, 1rem)',
            ':focus': {
              outline: '2px solid transparent',
              outlineOffset: '2px',
            },
            '& figure': {
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
            },
            '&.image-full': {
              display: 'grid',
            },
            '&.image-full:before': {
              position: 'relative',
              content: '""',
              zIndex: '10',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              opacity: '0.75',
              borderRadius: 'var(--rounded-box, 1rem)',
              gridColumnStart: '1',
              gridRowStart: '1',
            },
            '&.image-full > *': {
              gridColumnStart: '1',
              gridRowStart: '1',
            },
            '&.image-full > figure img': {
              height: '100%',
              objectFit: 'cover',
            },
            '&.image-full > .card-body': {
              position: 'relative',
              zIndex: '20',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
            },
            ':focus-visible': {
              outline: '2px solid currentColor',
              outlineOffset: '2px',
            },
            '&.bordered': {
              borderWidth: '1px',
              '--tw-border-opacity': '1',
              borderColor:
                'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
            },
            '&.compact .card-body': {
              padding: '1rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
            },
            width: '24rem',
            backgroundColor: 'hsl(var(--b1))',
            '--tw-shadow':
              '0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)',
            '--tw-shadow-colored':
              '0 20px 25px -5px var(--tw-shadow-color), 0 8px 10px -6px var(--tw-shadow-color)',
            boxShadow:
              'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
          }}
        >
          <figure>
            <img
              src="https://api.lorem.space/image/shoes?w=400&h=225"
              alt="Shoes"
            />
          </figure>
          <div
            css={{
              '.card.image-full > &': {
                position: 'relative',
                zIndex: '20',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.card.compact &': {
                padding: '1rem',
                fontSize: '0.875rem',
                lineHeight: '1.25rem',
              },
              display: 'flex',
              flex: '1 1 auto',
              flexDirection: 'column',
              padding: 'var(--padding-card, 2rem)',
              gap: '0.5rem',
              '& :where(p)': {
                flexGrow: '1',
              },
              '.card-compact &': {
                padding: '1rem',
                fontSize: '0.875rem',
                lineHeight: '1.25rem',
              },
              '.card-normal &': {
                padding: 'var(--padding-card, 2rem)',
                fontSize: '1rem',
                lineHeight: '1.5rem',
              },
            }}
          >
            <h2
              css={{
                display: 'flex',
                alignItems: 'center',
                gap: '0.5rem',
                fontSize: '1.25rem',
                lineHeight: '1.75rem',
                fontWeight: '600',
                '.card-compact &': {
                  marginBottom: '0.25rem',
                },
                '.card-normal &': {
                  marginBottom: '0.75rem',
                },
              }}
            >
              Shoes!
            </h2>
            <p>If a dog chews shoes whose shoes does he choose?</p>
            <div
              css={{
                display: 'flex',
                flexWrap: 'wrap',
                alignItems: 'flex-start',
                gap: '0.5rem',
                justifyContent: 'flex-end',
              }}
            >
              <button
                css={{
                  display: 'inline-flex',
                  flexShrink: '0',
                  cursor: 'pointer',
                  userSelect: 'none',
                  flexWrap: 'wrap',
                  alignItems: 'center',
                  justifyContent: 'center',
                  borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                  textAlign: 'center',
                  transitionProperty:
                    'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
                  transitionDuration: '200ms',
                  transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
                  borderRadius: 'var(--rounded-btn, 0.5rem)',
                  height: '3rem',
                  paddingLeft: '1rem',
                  paddingRight: '1rem',
                  fontSize: '0.875rem',
                  lineHeight: '1em',
                  minHeight: '3rem',
                  fontWeight: '600',
                  textTransform: 'var(--btn-text-case, uppercase)',
                  textDecorationLine: 'none',
                  borderWidth: 'var(--border-btn, 1px)',
                  animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
                  '--tw-border-opacity': '1',
                  '--tw-bg-opacity': '1',
                  backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                  '--tw-text-opacity': '1',
                  color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  '&.loading': {
                    pointerEvents: 'none',
                  },
                  '&.loading:hover': {
                    pointerEvents: 'none',
                  },
                  '&.loading:before': {
                    marginRight: '0.5rem',
                    height: '1rem',
                    width: '1rem',
                    borderRadius: '9999px',
                    borderWidth: '2px',
                    animation: 'spin 2s linear infinite',
                    content: '""',
                    borderTopColor: 'transparent',
                    borderLeftColor: 'transparent',
                    borderBottomColor: 'currentColor',
                    borderRightColor: 'currentColor',
                  },
                  ':active:hover': {
                    animation: 'none',
                    transform: 'scale(var(--btn-focus-scale, 0.95))',
                  },
                  ':active:focus': {
                    animation: 'none',
                    transform: 'scale(var(--btn-focus-scale, 0.95))',
                  },
                  ':hover': {
                    '--tw-border-opacity': '1',
                    borderColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                  },
                  ':focus-visible': {
                    outline: '2px solid hsl(var(--p))',
                    outlineOffset: '2px',
                  },
                  '&.glass:hover': {
                    '--glass-opacity': '25%',
                    '--glass-border-opacity': '15%',
                  },
                  '&.glass.btn-active': {
                    '--glass-opacity': '25%',
                    '--glass-border-opacity': '15%',
                  },
                  '&.glass:focus-visible': {
                    outline: '2px solid 0 0 2px currentColor',
                  },
                  '&.loading.btn-square:before': {
                    marginRight: '0px',
                  },
                  '&.loading.btn-circle:before': {
                    marginRight: '0px',
                  },
                  '&.loading.btn-xl:before': {
                    height: '1.25rem',
                    width: '1.25rem',
                  },
                  '&.loading.btn-lg:before': {
                    height: '1.25rem',
                    width: '1.25rem',
                  },
                  '&.loading.btn-sm:before': {
                    height: '0.75rem',
                    width: '0.75rem',
                  },
                  '&.loading.btn-xs:before': {
                    height: '0.75rem',
                    width: '0.75rem',
                  },
                  '.btn-group > input[type="radio"]:checked&': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                  '.btn-group > input[type="radio"]:checked&:focus-visible': {
                    outline: '2px solid hsl(var(--p))',
                  },
                  '.btn-group > &:not(:first-of-type)': {
                    marginLeft: '-1px',
                    borderTopLeftRadius: '0px',
                    borderBottomLeftRadius: '0px',
                  },
                  '.btn-group > &:not(:last-of-type)': {
                    borderTopRightRadius: '0px',
                    borderBottomRightRadius: '0px',
                  },
                  '@media (prefers-reduced-motion: reduce)': {
                    '&.loading:before': {
                      animation: 'spin 10s linear infinite',
                    },
                  },
                  '.btn-outline& .badge': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                  '.btn-outline&:hover .badge': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--p) / var(--tw-text-opacity))',
                  },
                  '.btn-outline&:hover .badge.outline': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                  '.drawer-toggle:focus-visible ~ .drawer-content .drawer-button&':
                    {
                      outline: '2px solid hsl(var(--p))',
                    },
                  '.btn-outline& .badge-outline': {
                    '--tw-border-opacity': '1',
                    borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                    backgroundColor: 'transparent',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--p) / var(--tw-text-opacity))',
                  },
                  '.btn-outline&': {
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--p) / var(--tw-text-opacity))',
                  },
                  '.btn-outline&:hover': {
                    '--tw-border-opacity': '1',
                    borderColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                  },
                  '&.btn-active': {
                    '--tw-border-opacity': '1',
                    borderColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                    '--tw-bg-opacity': '1',
                    backgroundColor:
                      'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                  },
                }}
              >
                Buy Now
              </button>
            </div>
          </div>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
          gap: '0.5rem',
          display: 'grid',
        }}
      >
        <div
          css={{
            display: 'flex',
            overflowX: 'scroll',
            scrollSnapType: 'x mandatory',
            scrollBehavior: 'smooth',
            MsOverflowStyle: 'none',
            scrollbarWidth: 'none',
            '::-webkit-scrollbar': {
              display: 'none',
            },
            width: '100%',
          }}
        >
          <div
            id="item1"
            css={{
              boxSizing: 'content-box',
              display: 'flex',
              flex: 'none',
              scrollSnapAlign: 'start',
              '.carousel-center &': {
                scrollSnapAlign: 'center',
              },
              '.carousel-end &': {
                scrollSnapAlign: 'end',
              },
              width: '100%',
            }}
          >
            <img
              src="https://api.lorem.space/image/car?w=800&h=200&hash=8B7BCDC2"
              css={{
                width: '100%',
              }}
            />
          </div>
          <div
            id="item2"
            css={{
              boxSizing: 'content-box',
              display: 'flex',
              flex: 'none',
              scrollSnapAlign: 'start',
              '.carousel-center &': {
                scrollSnapAlign: 'center',
              },
              '.carousel-end &': {
                scrollSnapAlign: 'end',
              },
              width: '100%',
            }}
          >
            <img
              src="https://api.lorem.space/image/car?w=800&h=200&hash=500B67FB"
              css={{
                width: '100%',
              }}
            />
          </div>
          <div
            id="item3"
            css={{
              boxSizing: 'content-box',
              display: 'flex',
              flex: 'none',
              scrollSnapAlign: 'start',
              '.carousel-center &': {
                scrollSnapAlign: 'center',
              },
              '.carousel-end &': {
                scrollSnapAlign: 'end',
              },
              width: '100%',
            }}
          >
            <img
              src="https://api.lorem.space/image/car?w=800&h=200&hash=A89D0DE6"
              css={{
                width: '100%',
              }}
            />
          </div>
          <div
            id="item4"
            css={{
              boxSizing: 'content-box',
              display: 'flex',
              flex: 'none',
              scrollSnapAlign: 'start',
              '.carousel-center &': {
                scrollSnapAlign: 'center',
              },
              '.carousel-end &': {
                scrollSnapAlign: 'end',
              },
              width: '100%',
            }}
          >
            <img
              src="https://api.lorem.space/image/car?w=800&h=200&hash=225E6693"
              css={{
                width: '100%',
              }}
            />
          </div>
        </div>
        <div
          css={{
            display: 'flex',
            justifyContent: 'center',
            width: '100%',
            paddingTop: '0.5rem',
            paddingBottom: '0.5rem',
            gap: '0.5rem',
          }}
        >
          <a
            href="#item1"
            css={{
              display: 'inline-flex',
              flexShrink: '0',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              borderColor: 'hsl(var(--n) / var(--tw-border-opacity))',
              textAlign: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              borderRadius: 'var(--rounded-btn, 0.5rem)',
              height: '1.5rem',
              paddingLeft: '0.5rem',
              paddingRight: '0.5rem',
              fontSize: '0.75rem',
              lineHeight: '1em',
              minHeight: '1.5rem',
              fontWeight: '600',
              textTransform: 'var(--btn-text-case, uppercase)',
              textDecorationLine: 'none',
              borderWidth: 'var(--border-btn, 1px)',
              animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              '&.loading': {
                pointerEvents: 'none',
              },
              '&.loading:hover': {
                pointerEvents: 'none',
              },
              '&.loading:before': {
                marginRight: '0.5rem',
                height: '1rem',
                width: '1rem',
                borderRadius: '9999px',
                borderWidth: '2px',
                animation: 'spin 2s linear infinite',
                content: '""',
                borderTopColor: 'transparent',
                borderLeftColor: 'transparent',
                borderBottomColor: 'currentColor',
                borderRightColor: 'currentColor',
              },
              ':active:hover': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':active:focus': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':hover': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
              },
              ':focus-visible': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '&.glass:hover': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass.btn-active': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass:focus-visible': {
                outline: '2px solid 0 0 2px currentColor',
              },
              '&.loading.btn-square:before': {
                marginRight: '0px',
              },
              '&.loading.btn-circle:before': {
                marginRight: '0px',
              },
              '&.loading.btn-xl:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-lg:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-sm:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '&.loading.btn-xs:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-group > input[type="radio"]:checked&': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-group > input[type="radio"]:checked&:focus-visible': {
                outline: '2px solid hsl(var(--p))',
              },
              '.btn-group > &:not(:first-of-type)': {
                marginLeft: '-1px',
                borderTopLeftRadius: '0px',
                borderBottomLeftRadius: '0px',
              },
              '.btn-group > &:not(:last-of-type)': {
                borderTopRightRadius: '0px',
                borderBottomRightRadius: '0px',
              },
              '@media (prefers-reduced-motion: reduce)': {
                '&.loading:before': {
                  animation: 'spin 10s linear infinite',
                },
              },
              '.btn.loading&:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-square': {
                height: '1.5rem',
                width: '1.5rem',
                padding: '0px',
              },
              '.btn-circle': {
                height: '1.5rem',
                width: '1.5rem',
                borderRadius: '9999px',
                padding: '0px',
              },
            }}
          >
            1
          </a>
          <a
            href="#item2"
            css={{
              display: 'inline-flex',
              flexShrink: '0',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              borderColor: 'hsl(var(--n) / var(--tw-border-opacity))',
              textAlign: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              borderRadius: 'var(--rounded-btn, 0.5rem)',
              height: '1.5rem',
              paddingLeft: '0.5rem',
              paddingRight: '0.5rem',
              fontSize: '0.75rem',
              lineHeight: '1em',
              minHeight: '1.5rem',
              fontWeight: '600',
              textTransform: 'var(--btn-text-case, uppercase)',
              textDecorationLine: 'none',
              borderWidth: 'var(--border-btn, 1px)',
              animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              '&.loading': {
                pointerEvents: 'none',
              },
              '&.loading:hover': {
                pointerEvents: 'none',
              },
              '&.loading:before': {
                marginRight: '0.5rem',
                height: '1rem',
                width: '1rem',
                borderRadius: '9999px',
                borderWidth: '2px',
                animation: 'spin 2s linear infinite',
                content: '""',
                borderTopColor: 'transparent',
                borderLeftColor: 'transparent',
                borderBottomColor: 'currentColor',
                borderRightColor: 'currentColor',
              },
              ':active:hover': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':active:focus': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':hover': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
              },
              ':focus-visible': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '&.glass:hover': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass.btn-active': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass:focus-visible': {
                outline: '2px solid 0 0 2px currentColor',
              },
              '&.loading.btn-square:before': {
                marginRight: '0px',
              },
              '&.loading.btn-circle:before': {
                marginRight: '0px',
              },
              '&.loading.btn-xl:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-lg:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-sm:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '&.loading.btn-xs:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-group > input[type="radio"]:checked&': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-group > input[type="radio"]:checked&:focus-visible': {
                outline: '2px solid hsl(var(--p))',
              },
              '.btn-group > &:not(:first-of-type)': {
                marginLeft: '-1px',
                borderTopLeftRadius: '0px',
                borderBottomLeftRadius: '0px',
              },
              '.btn-group > &:not(:last-of-type)': {
                borderTopRightRadius: '0px',
                borderBottomRightRadius: '0px',
              },
              '@media (prefers-reduced-motion: reduce)': {
                '&.loading:before': {
                  animation: 'spin 10s linear infinite',
                },
              },
              '.btn.loading&:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-square': {
                height: '1.5rem',
                width: '1.5rem',
                padding: '0px',
              },
              '.btn-circle': {
                height: '1.5rem',
                width: '1.5rem',
                borderRadius: '9999px',
                padding: '0px',
              },
            }}
          >
            2
          </a>
          <a
            href="#item3"
            css={{
              display: 'inline-flex',
              flexShrink: '0',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              borderColor: 'hsl(var(--n) / var(--tw-border-opacity))',
              textAlign: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              borderRadius: 'var(--rounded-btn, 0.5rem)',
              height: '1.5rem',
              paddingLeft: '0.5rem',
              paddingRight: '0.5rem',
              fontSize: '0.75rem',
              lineHeight: '1em',
              minHeight: '1.5rem',
              fontWeight: '600',
              textTransform: 'var(--btn-text-case, uppercase)',
              textDecorationLine: 'none',
              borderWidth: 'var(--border-btn, 1px)',
              animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              '&.loading': {
                pointerEvents: 'none',
              },
              '&.loading:hover': {
                pointerEvents: 'none',
              },
              '&.loading:before': {
                marginRight: '0.5rem',
                height: '1rem',
                width: '1rem',
                borderRadius: '9999px',
                borderWidth: '2px',
                animation: 'spin 2s linear infinite',
                content: '""',
                borderTopColor: 'transparent',
                borderLeftColor: 'transparent',
                borderBottomColor: 'currentColor',
                borderRightColor: 'currentColor',
              },
              ':active:hover': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':active:focus': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':hover': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
              },
              ':focus-visible': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '&.glass:hover': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass.btn-active': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass:focus-visible': {
                outline: '2px solid 0 0 2px currentColor',
              },
              '&.loading.btn-square:before': {
                marginRight: '0px',
              },
              '&.loading.btn-circle:before': {
                marginRight: '0px',
              },
              '&.loading.btn-xl:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-lg:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-sm:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '&.loading.btn-xs:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-group > input[type="radio"]:checked&': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-group > input[type="radio"]:checked&:focus-visible': {
                outline: '2px solid hsl(var(--p))',
              },
              '.btn-group > &:not(:first-of-type)': {
                marginLeft: '-1px',
                borderTopLeftRadius: '0px',
                borderBottomLeftRadius: '0px',
              },
              '.btn-group > &:not(:last-of-type)': {
                borderTopRightRadius: '0px',
                borderBottomRightRadius: '0px',
              },
              '@media (prefers-reduced-motion: reduce)': {
                '&.loading:before': {
                  animation: 'spin 10s linear infinite',
                },
              },
              '.btn.loading&:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-square': {
                height: '1.5rem',
                width: '1.5rem',
                padding: '0px',
              },
              '.btn-circle': {
                height: '1.5rem',
                width: '1.5rem',
                borderRadius: '9999px',
                padding: '0px',
              },
            }}
          >
            3
          </a>
          <a
            href="#item4"
            css={{
              display: 'inline-flex',
              flexShrink: '0',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              borderColor: 'hsl(var(--n) / var(--tw-border-opacity))',
              textAlign: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              borderRadius: 'var(--rounded-btn, 0.5rem)',
              height: '1.5rem',
              paddingLeft: '0.5rem',
              paddingRight: '0.5rem',
              fontSize: '0.75rem',
              lineHeight: '1em',
              minHeight: '1.5rem',
              fontWeight: '600',
              textTransform: 'var(--btn-text-case, uppercase)',
              textDecorationLine: 'none',
              borderWidth: 'var(--border-btn, 1px)',
              animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              '&.loading': {
                pointerEvents: 'none',
              },
              '&.loading:hover': {
                pointerEvents: 'none',
              },
              '&.loading:before': {
                marginRight: '0.5rem',
                height: '1rem',
                width: '1rem',
                borderRadius: '9999px',
                borderWidth: '2px',
                animation: 'spin 2s linear infinite',
                content: '""',
                borderTopColor: 'transparent',
                borderLeftColor: 'transparent',
                borderBottomColor: 'currentColor',
                borderRightColor: 'currentColor',
              },
              ':active:hover': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':active:focus': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':hover': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
              },
              ':focus-visible': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '&.glass:hover': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass.btn-active': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass:focus-visible': {
                outline: '2px solid 0 0 2px currentColor',
              },
              '&.loading.btn-square:before': {
                marginRight: '0px',
              },
              '&.loading.btn-circle:before': {
                marginRight: '0px',
              },
              '&.loading.btn-xl:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-lg:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-sm:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '&.loading.btn-xs:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-group > input[type="radio"]:checked&': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-group > input[type="radio"]:checked&:focus-visible': {
                outline: '2px solid hsl(var(--p))',
              },
              '.btn-group > &:not(:first-of-type)': {
                marginLeft: '-1px',
                borderTopLeftRadius: '0px',
                borderBottomLeftRadius: '0px',
              },
              '.btn-group > &:not(:last-of-type)': {
                borderTopRightRadius: '0px',
                borderBottomRightRadius: '0px',
              },
              '@media (prefers-reduced-motion: reduce)': {
                '&.loading:before': {
                  animation: 'spin 10s linear infinite',
                },
              },
              '.btn.loading&:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-square': {
                height: '1.5rem',
                width: '1.5rem',
                padding: '0px',
              },
              '.btn-circle': {
                height: '1.5rem',
                width: '1.5rem',
                borderRadius: '9999px',
                padding: '0px',
              },
            }}
          >
            4
          </a>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            position: 'relative',
            display: 'inline-flex',
            width: 'max-content',
            '& :where(.indicator-item)': {
              zIndex: '1',
              position: 'absolute',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              right: '0px',
              left: 'auto',
              top: '0px',
              bottom: 'auto',
              '--tw-translate-x': '50%',
              '--tw-translate-y': '-50%',
            },
            '& :where(.indicator-item.indicator-start)': {
              right: 'auto',
              left: '0px',
              '--tw-translate-x': '-50%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
            '& :where(.indicator-item.indicator-center)': {
              right: '50%',
              left: '50%',
              '--tw-translate-x': '-50%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
            '& :where(.indicator-item.indicator-end)': {
              right: '0px',
              left: 'auto',
              '--tw-translate-x': '50%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
            '& :where(.indicator-item.indicator-bottom)': {
              top: 'auto',
              bottom: '0px',
              '--tw-translate-y': '50%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
            '& :where(.indicator-item.indicator-middle)': {
              top: '50%',
              bottom: '50%',
              '--tw-translate-y': '-50%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
            '& :where(.indicator-item.indicator-top)': {
              top: '0px',
              bottom: 'auto',
              '--tw-translate-y': '-50%',
              transform:
                'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            },
          }}
          className="indicator"
        >
          <span
            className="indicator-item"
            css={{
              '.indicator ': {
                zIndex: '1',
                position: 'absolute',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                top: '0px',
                bottom: 'auto',
                '--tw-translate-x': '50%',
                '--tw-translate-y': '-50%',
              },
              '.indicator :where(&.indicator-start)': {
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-center)': {
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-end)': {
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-bottom)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-middle)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-top)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(.indicator-item&)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
              },
              display: 'inline-flex',
              alignItems: 'center',
              justifyContent: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              height: '1.25rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
              width: 'fit-content',
              paddingLeft: '0.563rem',
              paddingRight: '0.563rem',
              borderWidth: '1px',
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              borderRadius: 'var(--rounded-badge, 1.9rem)',
              '.btn-outline &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.btn-outline &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                backgroundColor: 'transparent',
              },
              '.btn-outline:hover &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              '.btn-outline:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--p) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--a) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.badge-outline&': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
            }}
          >
            top+start
          </span>
          <span
            className="indicator-item indicator-center"
            css={{
              '.indicator ': {
                zIndex: '1',
                position: 'absolute',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                top: '0px',
                bottom: 'auto',
                '--tw-translate-x': '50%',
                '--tw-translate-y': '-50%',
              },
              '.indicator :where(&.indicator-start)': {
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-center)': {
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-end)': {
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-bottom)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-middle)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-top)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(.indicator-item&)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
              },
              display: 'inline-flex',
              alignItems: 'center',
              justifyContent: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              height: '1.25rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
              width: 'fit-content',
              paddingLeft: '0.563rem',
              paddingRight: '0.563rem',
              borderWidth: '1px',
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              borderRadius: 'var(--rounded-badge, 1.9rem)',
              '.btn-outline &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.btn-outline &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                backgroundColor: 'transparent',
              },
              '.btn-outline:hover &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              '.btn-outline:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--p) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--a) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.badge-outline&': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
            }}
          >
            top+center
          </span>
          <span
            className="indicator-item"
            css={{
              '.indicator ': {
                zIndex: '1',
                position: 'absolute',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                top: '0px',
                bottom: 'auto',
                '--tw-translate-x': '50%',
                '--tw-translate-y': '-50%',
              },
              '.indicator :where(&.indicator-start)': {
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-center)': {
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-end)': {
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-bottom)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-middle)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-top)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(.indicator-item&)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
              },
              display: 'inline-flex',
              alignItems: 'center',
              justifyContent: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              height: '1.25rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
              width: 'fit-content',
              paddingLeft: '0.563rem',
              paddingRight: '0.563rem',
              borderWidth: '1px',
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              borderRadius: 'var(--rounded-badge, 1.9rem)',
              '.btn-outline &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.btn-outline &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                backgroundColor: 'transparent',
              },
              '.btn-outline:hover &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              '.btn-outline:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--p) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--a) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.badge-outline&': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
            }}
          >
            top+end
          </span>
          <span
            className="indicator-item"
            css={{
              '.indicator ': {
                zIndex: '1',
                position: 'absolute',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                top: '0px',
                bottom: 'auto',
                '--tw-translate-x': '50%',
                '--tw-translate-y': '-50%',
              },
              '.indicator :where(&.indicator-start)': {
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-center)': {
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-end)': {
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-bottom)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-middle)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-top)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(.indicator-item&)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
              },
              display: 'inline-flex',
              alignItems: 'center',
              justifyContent: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              height: '1.25rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
              width: 'fit-content',
              paddingLeft: '0.563rem',
              paddingRight: '0.563rem',
              borderWidth: '1px',
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              borderRadius: 'var(--rounded-badge, 1.9rem)',
              '.btn-outline &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.btn-outline &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                backgroundColor: 'transparent',
              },
              '.btn-outline:hover &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              '.btn-outline:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--p) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--a) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.badge-outline&': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
            }}
          >
            middle+start
          </span>
          <span
            className="indicator-item"
            css={{
              '.indicator ': {
                zIndex: '1',
                position: 'absolute',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                top: '0px',
                bottom: 'auto',
                '--tw-translate-x': '50%',
                '--tw-translate-y': '-50%',
              },
              '.indicator :where(&.indicator-start)': {
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-center)': {
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-end)': {
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-bottom)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-middle)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-top)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(.indicator-item&)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
              },
              display: 'inline-flex',
              alignItems: 'center',
              justifyContent: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              height: '1.25rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
              width: 'fit-content',
              paddingLeft: '0.563rem',
              paddingRight: '0.563rem',
              borderWidth: '1px',
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              borderRadius: 'var(--rounded-badge, 1.9rem)',
              '.btn-outline &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.btn-outline &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                backgroundColor: 'transparent',
              },
              '.btn-outline:hover &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              '.btn-outline:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--p) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--a) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.badge-outline&': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
            }}
          >
            middle+center
          </span>
          <span
            className="indicator-item"
            css={{
              '.indicator ': {
                zIndex: '1',
                position: 'absolute',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                top: '0px',
                bottom: 'auto',
                '--tw-translate-x': '50%',
                '--tw-translate-y': '-50%',
              },
              '.indicator :where(&.indicator-start)': {
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-center)': {
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-end)': {
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-bottom)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-middle)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-top)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(.indicator-item&)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
              },
              display: 'inline-flex',
              alignItems: 'center',
              justifyContent: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              height: '1.25rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
              width: 'fit-content',
              paddingLeft: '0.563rem',
              paddingRight: '0.563rem',
              borderWidth: '1px',
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              borderRadius: 'var(--rounded-badge, 1.9rem)',
              '.btn-outline &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.btn-outline &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                backgroundColor: 'transparent',
              },
              '.btn-outline:hover &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              '.btn-outline:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--p) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--a) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.badge-outline&': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
            }}
          >
            middle+end
          </span>
          <span
            className="indicator-item"
            css={{
              '.indicator ': {
                zIndex: '1',
                position: 'absolute',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                top: '0px',
                bottom: 'auto',
                '--tw-translate-x': '50%',
                '--tw-translate-y': '-50%',
              },
              '.indicator :where(&.indicator-start)': {
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-center)': {
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-end)': {
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-bottom)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-middle)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-top)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(.indicator-item&)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
              },
              display: 'inline-flex',
              alignItems: 'center',
              justifyContent: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              height: '1.25rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
              width: 'fit-content',
              paddingLeft: '0.563rem',
              paddingRight: '0.563rem',
              borderWidth: '1px',
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              borderRadius: 'var(--rounded-badge, 1.9rem)',
              '.btn-outline &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.btn-outline &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                backgroundColor: 'transparent',
              },
              '.btn-outline:hover &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              '.btn-outline:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--p) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--a) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.badge-outline&': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
            }}
          >
            bott0m+strt
          </span>
          <span
            className="indicator-item"
            css={{
              '.indicator ': {
                zIndex: '1',
                position: 'absolute',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                top: '0px',
                bottom: 'auto',
                '--tw-translate-x': '50%',
                '--tw-translate-y': '-50%',
              },
              '.indicator :where(&.indicator-start)': {
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-center)': {
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-end)': {
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-bottom)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-middle)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-top)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(.indicator-item&)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
              },
              display: 'inline-flex',
              alignItems: 'center',
              justifyContent: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              height: '1.25rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
              width: 'fit-content',
              paddingLeft: '0.563rem',
              paddingRight: '0.563rem',
              borderWidth: '1px',
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              borderRadius: 'var(--rounded-badge, 1.9rem)',
              '.btn-outline &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.btn-outline &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                backgroundColor: 'transparent',
              },
              '.btn-outline:hover &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              '.btn-outline:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--p) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--a) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.badge-outline&': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
            }}
          >
            bottom+center
          </span>
          <span
            className="indicator-item"
            css={{
              '.indicator ': {
                zIndex: '1',
                position: 'absolute',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                top: '0px',
                bottom: 'auto',
                '--tw-translate-x': '50%',
                '--tw-translate-y': '-50%',
              },
              '.indicator :where(&.indicator-start)': {
                right: 'auto',
                left: '0px',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-center)': {
                right: '50%',
                left: '50%',
                '--tw-translate-x': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-end)': {
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-bottom)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-middle)': {
                top: '50%',
                bottom: '50%',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(&.indicator-top)': {
                top: '0px',
                bottom: 'auto',
                '--tw-translate-y': '-50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
              },
              '.indicator :where(.indicator-item&)': {
                top: 'auto',
                bottom: '0px',
                '--tw-translate-y': '50%',
                transform:
                  'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
                right: '0px',
                left: 'auto',
                '--tw-translate-x': '50%',
              },
              display: 'inline-flex',
              alignItems: 'center',
              justifyContent: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              height: '1.25rem',
              fontSize: '0.875rem',
              lineHeight: '1.25rem',
              width: 'fit-content',
              paddingLeft: '0.563rem',
              paddingRight: '0.563rem',
              borderWidth: '1px',
              '--tw-border-opacity': '1',
              borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              borderRadius: 'var(--rounded-badge, 1.9rem)',
              '.btn-outline &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--s) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--s) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--a) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--a) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.btn-outline &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                backgroundColor: 'transparent',
              },
              '.btn-outline:hover &': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--bc) / var(--tw-text-opacity))',
              },
              '.btn-outline:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--pc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--p) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-primary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--pc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--sc) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-secondary:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--sc) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--sf, var(--s)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--sc) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--ac) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--a) / var(--tw-text-opacity))',
              },
              '.btn-outline.btn-accent:hover &.outline': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--ac) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--af, var(--a)) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--ac) / var(--tw-text-opacity))',
              },
              '.badge-outline&': {
                '--tw-text-opacity': '1',
                color: 'hsl(var(--s) / var(--tw-text-opacity))',
              },
            }}
          >
            bottom+end
          </span>
          <div
            css={{
              display: 'grid',
              width: '15rem',
              height: '8rem',
              backgroundColor: 'hsl(var(--b3, var(--b2)))',
              placeItems: 'center',
            }}
          >
            content
          </div>
        </div>
      </div>
      <div
        css={{
          padding: '1.25rem',
          margin: '1.25rem',
        }}
      >
        <div
          css={{
            display: 'grid',
            width: '100%',
            placeItems: 'center',
            backgroundSize: 'cover',
            backgroundPosition: 'center',
            '& > *': {
              gridColumnStart: '1',
              gridRowStart: '1',
            },
            minHeight: '100vh',
            backgroundColor: 'hsl(var(--b2, var(--b1)))',
          }}
        >
          <div
            css={{
              zIndex: '0',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              maxWidth: '80rem',
              gap: '1rem',
              padding: '1rem',
              flexDirection: 'column',
              '@media (min-width: 1024px)': {
                flexDirection: 'row-reverse',
              },
            }}
          >
            <div
              css={{
                textAlign: 'center',
                '@media (min-width: 1024px)': {
                  textAlign: 'left',
                },
              }}
            >
              <h1
                css={{
                  fontSize: '3rem',
                  lineHeight: '1',
                  fontWeight: '700',
                }}
              >
                Login now!
              </h1>
              <p
                css={{
                  paddingTop: '1.5rem',
                  paddingBottom: '1.5rem',
                }}
              >
                Provident cupiditate voluptatem et in. Quaerat fugiat ut
                assumenda excepturi exercitationem quasi. In deleniti eaque aut
                repudiandae et a id nisi.
              </p>
            </div>
            <div
              css={{
                position: 'relative',
                display: 'flex',
                flexDirection: 'column',
                overflow: 'hidden',
                borderRadius: 'var(--rounded-box, 1rem)',
                ':focus': {
                  outline: '2px solid transparent',
                  outlineOffset: '2px',
                },
                '& figure': {
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                },
                '&.image-full': {
                  display: 'grid',
                },
                '&.image-full:before': {
                  position: 'relative',
                  content: '""',
                  zIndex: '10',
                  '--tw-bg-opacity': '1',
                  backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
                  opacity: '0.75',
                  borderRadius: 'var(--rounded-box, 1rem)',
                  gridColumnStart: '1',
                  gridRowStart: '1',
                },
                '&.image-full > *': {
                  gridColumnStart: '1',
                  gridRowStart: '1',
                },
                '&.image-full > figure img': {
                  height: '100%',
                  objectFit: 'cover',
                },
                '&.image-full > .card-body': {
                  position: 'relative',
                  zIndex: '20',
                  '--tw-text-opacity': '1',
                  color: 'hsl(var(--nc) / var(--tw-text-opacity))',
                },
                ':focus-visible': {
                  outline: '2px solid currentColor',
                  outlineOffset: '2px',
                },
                '&.bordered': {
                  borderWidth: '1px',
                  '--tw-border-opacity': '1',
                  borderColor:
                    'hsl(var(--b2, var(--b1)) / var(--tw-border-opacity))',
                },
                '&.compact .card-body': {
                  padding: '1rem',
                  fontSize: '0.875rem',
                  lineHeight: '1.25rem',
                },
                flexShrink: '0',
                width: '100%',
                maxWidth: '24rem',
                '--tw-shadow': '0 25px 50px -12px rgb(0 0 0 / 0.25)',
                '--tw-shadow-colored':
                  '0 25px 50px -12px var(--tw-shadow-color)',
                boxShadow:
                  'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
                backgroundColor: 'hsl(var(--b1))',
              }}
            >
              <div
                css={{
                  '.card.image-full > &': {
                    position: 'relative',
                    zIndex: '20',
                    '--tw-text-opacity': '1',
                    color: 'hsl(var(--nc) / var(--tw-text-opacity))',
                  },
                  '.card.compact &': {
                    padding: '1rem',
                    fontSize: '0.875rem',
                    lineHeight: '1.25rem',
                  },
                  display: 'flex',
                  flex: '1 1 auto',
                  flexDirection: 'column',
                  padding: 'var(--padding-card, 2rem)',
                  gap: '0.5rem',
                  '& :where(p)': {
                    flexGrow: '1',
                  },
                  '.card-compact &': {
                    padding: '1rem',
                    fontSize: '0.875rem',
                    lineHeight: '1.25rem',
                  },
                  '.card-normal &': {
                    padding: 'var(--padding-card, 2rem)',
                    fontSize: '1rem',
                    lineHeight: '1.5rem',
                  },
                }}
              >
                <div
                  css={{
                    display: 'flex',
                    flexDirection: 'column',
                  }}
                >
                  <label
                    css={{
                      display: 'flex',
                      userSelect: 'none',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      paddingLeft: '0.25rem',
                      paddingRight: '0.25rem',
                      paddingTop: '0.5rem',
                      paddingBottom: '0.5rem',
                      '& a:hover': {
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--bc) / var(--tw-text-opacity))',
                      },
                    }}
                  >
                    <span
                      css={{
                        fontSize: '0.875rem',
                        lineHeight: '1.25rem',
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--bc) / var(--tw-text-opacity))',
                      }}
                    >
                      Email
                    </span>
                  </label>
                  <input
                    type="text"
                    placeholder="email"
                    css={{
                      flexShrink: '1',
                      transitionProperty:
                        'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
                      transitionDuration: '200ms',
                      transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
                      height: '3rem',
                      paddingLeft: '1rem',
                      paddingRight: '1rem',
                      fontSize: '0.875rem',
                      lineHeight: '2',
                      borderWidth: '1px',
                      borderColor: 'hsl(var(--bc) / var(--tw-border-opacity))',
                      '--tw-border-opacity': '0.2',
                      '--tw-bg-opacity': '1',
                      backgroundColor: 'hsl(var(--b1) / var(--tw-bg-opacity))',
                      borderRadius: 'var(--rounded-btn, 0.5rem)',
                      '.input-group > &': {
                        borderRadius: '0px',
                      },
                      ':focus': {
                        outline: '2px solid hsla(var(--bc) / 0.2)',
                        outlineOffset: '2px',
                      },
                    }}
                  />
                </div>
                <div
                  css={{
                    display: 'flex',
                    flexDirection: 'column',
                  }}
                >
                  <label
                    css={{
                      display: 'flex',
                      userSelect: 'none',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      paddingLeft: '0.25rem',
                      paddingRight: '0.25rem',
                      paddingTop: '0.5rem',
                      paddingBottom: '0.5rem',
                      '& a:hover': {
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--bc) / var(--tw-text-opacity))',
                      },
                    }}
                  >
                    <span
                      css={{
                        fontSize: '0.875rem',
                        lineHeight: '1.25rem',
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--bc) / var(--tw-text-opacity))',
                      }}
                    >
                      Password
                    </span>
                  </label>
                  <input
                    type="text"
                    placeholder="password"
                    css={{
                      flexShrink: '1',
                      transitionProperty:
                        'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
                      transitionDuration: '200ms',
                      transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
                      height: '3rem',
                      paddingLeft: '1rem',
                      paddingRight: '1rem',
                      fontSize: '0.875rem',
                      lineHeight: '2',
                      borderWidth: '1px',
                      borderColor: 'hsl(var(--bc) / var(--tw-border-opacity))',
                      '--tw-border-opacity': '0.2',
                      '--tw-bg-opacity': '1',
                      backgroundColor: 'hsl(var(--b1) / var(--tw-bg-opacity))',
                      borderRadius: 'var(--rounded-btn, 0.5rem)',
                      '.input-group > &': {
                        borderRadius: '0px',
                      },
                      ':focus': {
                        outline: '2px solid hsla(var(--bc) / 0.2)',
                        outlineOffset: '2px',
                      },
                    }}
                  />
                  <label
                    css={{
                      display: 'flex',
                      userSelect: 'none',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      paddingLeft: '0.25rem',
                      paddingRight: '0.25rem',
                      paddingTop: '0.5rem',
                      paddingBottom: '0.5rem',
                      '& a:hover': {
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--bc) / var(--tw-text-opacity))',
                      },
                    }}
                  >
                    <a
                      href="#"
                      css={{
                        fontSize: '0.75rem',
                        lineHeight: '1rem',
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--bc) / var(--tw-text-opacity))',
                        cursor: 'pointer',
                        textDecorationLine: 'none',
                        ':focus': {
                          outline: '2px solid transparent',
                          outlineOffset: '2px',
                        },
                        ':focus-visible': {
                          outline: '2px solid currentColor',
                          outlineOffset: '2px',
                        },
                        ':hover': {
                          textDecorationLine: 'underline',
                        },
                      }}
                    >
                      Forgot password?
                    </a>
                  </label>
                </div>
                <div
                  css={{
                    display: 'flex',
                    flexDirection: 'column',
                    marginTop: '1.5rem',
                  }}
                >
                  <button
                    css={{
                      display: 'inline-flex',
                      flexShrink: '0',
                      cursor: 'pointer',
                      userSelect: 'none',
                      flexWrap: 'wrap',
                      alignItems: 'center',
                      justifyContent: 'center',
                      borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                      textAlign: 'center',
                      transitionProperty:
                        'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
                      transitionDuration: '200ms',
                      transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
                      borderRadius: 'var(--rounded-btn, 0.5rem)',
                      height: '3rem',
                      paddingLeft: '1rem',
                      paddingRight: '1rem',
                      fontSize: '0.875rem',
                      lineHeight: '1em',
                      minHeight: '3rem',
                      fontWeight: '600',
                      textTransform: 'var(--btn-text-case, uppercase)',
                      textDecorationLine: 'none',
                      borderWidth: 'var(--border-btn, 1px)',
                      animation:
                        'button-pop var(--animation-btn, 0.25s) ease-out',
                      '--tw-border-opacity': '1',
                      '--tw-bg-opacity': '1',
                      backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                      '--tw-text-opacity': '1',
                      color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                      '&.loading': {
                        pointerEvents: 'none',
                      },
                      '&.loading:hover': {
                        pointerEvents: 'none',
                      },
                      '&.loading:before': {
                        marginRight: '0.5rem',
                        height: '1rem',
                        width: '1rem',
                        borderRadius: '9999px',
                        borderWidth: '2px',
                        animation: 'spin 2s linear infinite',
                        content: '""',
                        borderTopColor: 'transparent',
                        borderLeftColor: 'transparent',
                        borderBottomColor: 'currentColor',
                        borderRightColor: 'currentColor',
                      },
                      ':active:hover': {
                        animation: 'none',
                        transform: 'scale(var(--btn-focus-scale, 0.95))',
                      },
                      ':active:focus': {
                        animation: 'none',
                        transform: 'scale(var(--btn-focus-scale, 0.95))',
                      },
                      ':hover': {
                        '--tw-border-opacity': '1',
                        borderColor:
                          'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                        '--tw-bg-opacity': '1',
                        backgroundColor:
                          'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                      },
                      ':focus-visible': {
                        outline: '2px solid hsl(var(--p))',
                        outlineOffset: '2px',
                      },
                      '&.glass:hover': {
                        '--glass-opacity': '25%',
                        '--glass-border-opacity': '15%',
                      },
                      '&.glass.btn-active': {
                        '--glass-opacity': '25%',
                        '--glass-border-opacity': '15%',
                      },
                      '&.glass:focus-visible': {
                        outline: '2px solid 0 0 2px currentColor',
                      },
                      '&.loading.btn-square:before': {
                        marginRight: '0px',
                      },
                      '&.loading.btn-circle:before': {
                        marginRight: '0px',
                      },
                      '&.loading.btn-xl:before': {
                        height: '1.25rem',
                        width: '1.25rem',
                      },
                      '&.loading.btn-lg:before': {
                        height: '1.25rem',
                        width: '1.25rem',
                      },
                      '&.loading.btn-sm:before': {
                        height: '0.75rem',
                        width: '0.75rem',
                      },
                      '&.loading.btn-xs:before': {
                        height: '0.75rem',
                        width: '0.75rem',
                      },
                      '.btn-group > input[type="radio"]:checked&': {
                        '--tw-border-opacity': '1',
                        borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                        '--tw-bg-opacity': '1',
                        backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                      },
                      '.btn-group > input[type="radio"]:checked&:focus-visible':
                        {
                          outline: '2px solid hsl(var(--p))',
                        },
                      '.btn-group > &:not(:first-of-type)': {
                        marginLeft: '-1px',
                        borderTopLeftRadius: '0px',
                        borderBottomLeftRadius: '0px',
                      },
                      '.btn-group > &:not(:last-of-type)': {
                        borderTopRightRadius: '0px',
                        borderBottomRightRadius: '0px',
                      },
                      '@media (prefers-reduced-motion: reduce)': {
                        '&.loading:before': {
                          animation: 'spin 10s linear infinite',
                        },
                      },
                      '.btn-outline& .badge': {
                        '--tw-border-opacity': '1',
                        borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                        '--tw-bg-opacity': '1',
                        backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                      },
                      '.btn-outline&:hover .badge': {
                        '--tw-border-opacity': '1',
                        borderColor:
                          'hsl(var(--pc) / var(--tw-border-opacity))',
                        '--tw-bg-opacity': '1',
                        backgroundColor:
                          'hsl(var(--pc) / var(--tw-bg-opacity))',
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--p) / var(--tw-text-opacity))',
                      },
                      '.btn-outline&:hover .badge.outline': {
                        '--tw-border-opacity': '1',
                        borderColor:
                          'hsl(var(--pc) / var(--tw-border-opacity))',
                        '--tw-bg-opacity': '1',
                        backgroundColor:
                          'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                      },
                      '.drawer-toggle:focus-visible ~ .drawer-content .drawer-button&':
                        {
                          outline: '2px solid hsl(var(--p))',
                        },
                      '.btn-outline& .badge-outline': {
                        '--tw-border-opacity': '1',
                        borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                        backgroundColor: 'transparent',
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--p) / var(--tw-text-opacity))',
                      },
                      '.btn-outline&': {
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--p) / var(--tw-text-opacity))',
                      },
                      '.btn-outline&:hover': {
                        '--tw-border-opacity': '1',
                        borderColor:
                          'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                        '--tw-bg-opacity': '1',
                        backgroundColor:
                          'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                        '--tw-text-opacity': '1',
                        color: 'hsl(var(--pc) / var(--tw-text-opacity))',
                      },
                      '&.btn-active': {
                        '--tw-border-opacity': '1',
                        borderColor:
                          'hsl(var(--pf, var(--p)) / var(--tw-border-opacity))',
                        '--tw-bg-opacity': '1',
                        backgroundColor:
                          'hsl(var(--pf, var(--p)) / var(--tw-bg-opacity))',
                      },
                    }}
                  >
                    Login
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <input
      type="checkbox"
      id="my-modal"
      css={{
        '&:checked + .modal': {
          pointerEvents: 'auto',
          visibility: 'visible',
          opacity: '1',
        },
        '&:checked + .modal .modal-box': {
          '--tw-translate-y': '0px',
          '--tw-scale-x': '1',
          '--tw-scale-y': '1',
          transform:
            'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
        },
        position: 'fixed',
        height: '0px',
        width: '0px',
        appearance: 'none',
        opacity: '0',
      }}
      className="modal-toggle"
    />
    <div
      css={{
        pointerEvents: 'none',
        visibility: 'hidden',
        position: 'fixed',
        top: '0px',
        right: '0px',
        bottom: '0px',
        left: '0px',
        display: 'flex',
        justifyContent: 'center',
        opacity: '0',
        zIndex: '999',
        backgroundColor: 'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
        '--tw-bg-opacity': '0.4',
        transitionDuration: '200ms',
        transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
        transitionProperty: 'transform, opacity',
        overflowY: 'hidden',
        overscrollBehavior: 'contain',
        alignItems: 'center',
        ':target': {
          pointerEvents: 'auto',
          visibility: 'visible',
          opacity: '1',
        },
        '.modal-toggle:checked + &': {
          pointerEvents: 'auto',
          visibility: 'visible',
          opacity: '1',
        },
        '.modal-toggle:checked + & .modal-box': {
          '--tw-translate-y': '0px',
          '--tw-scale-x': '1',
          '--tw-scale-y': '1',
          transform:
            'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
        },
        '&:target .modal-box': {
          '--tw-translate-y': '0px',
          '--tw-scale-x': '1',
          '--tw-scale-y': '1',
          transform:
            'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
        },
      }}
      className="modal"
    >
      <div
        css={{
          '.modal-open &': {
            '--tw-translate-y': '0px',
            '--tw-scale-x': '1',
            '--tw-scale-y': '1',
            transform:
              'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
          },
          '.modal-toggle:checked + .modal &': {
            '--tw-translate-y': '0px',
            '--tw-scale-x': '1',
            '--tw-scale-y': '1',
            transform:
              'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
          },
          '.modal:target &': {
            '--tw-translate-y': '0px',
            '--tw-scale-x': '1',
            '--tw-scale-y': '1',
            transform:
              'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
          },
          maxHeight: 'calc(100vh - 5em)',
          '--tw-bg-opacity': '1',
          backgroundColor: 'hsl(var(--b1) / var(--tw-bg-opacity))',
          padding: '1.5rem',
          transitionProperty:
            'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
          transitionDuration: '200ms',
          transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
          width: '91.666667%',
          maxWidth: '32rem',
          '--tw-scale-x': '.9',
          '--tw-scale-y': '.9',
          transform:
            'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
          borderTopLeftRadius: 'var(--rounded-box, 1rem)',
          borderTopRightRadius: 'var(--rounded-box, 1rem)',
          borderBottomLeftRadius: 'var(--rounded-box, 1rem)',
          borderBottomRightRadius: 'var(--rounded-box, 1rem)',
          boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.25)',
          overflowY: 'auto',
          overscrollBehavior: 'contain',
          '.modal-bottom ': {
            width: '100%',
            maxWidth: 'none',
            '--tw-translate-y': '2.5rem',
            '--tw-scale-x': '1',
            '--tw-scale-y': '1',
            transform:
              'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            borderBottomRightRadius: '0px',
            borderBottomLeftRadius: '0px',
          },
          '.modal-middle ': {
            width: '91.666667%',
            maxWidth: '32rem',
            '--tw-translate-y': '0px',
            '--tw-scale-x': '.9',
            '--tw-scale-y': '.9',
            transform:
              'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
            borderBottomLeftRadius: 'var(--rounded-box, 1rem)',
            borderBottomRightRadius: 'var(--rounded-box, 1rem)',
          },
        }}
      >
        <h3
          css={{
            fontWeight: '700',
            fontSize: '1.125rem',
            lineHeight: '1.75rem',
          }}
        >
          Congratulations random Interner user!
        </h3>
        <p
          css={{
            paddingTop: '1rem',
            paddingBottom: '1rem',
          }}
        >
          You've been selected for a chance to get one year of subscription to
          use Wikipedia for free!
        </p>
        <div
          css={{
            display: 'flex',
            marginTop: '1.5rem',
            justifyContent: 'flex-end',
            '& > :not([hidden]) ~ :not([hidden])': {
              '--tw-space-x-reverse': '0',
              marginRight: 'calc(0.5rem * var(--tw-space-x-reverse))',
              marginLeft: 'calc(0.5rem * calc(1 - var(--tw-space-x-reverse)))',
            },
          }}
          className="modal-action"
        >
          <label
            htmlFor="my-modal"
            css={{
              display: 'inline-flex',
              flexShrink: '0',
              cursor: 'pointer',
              userSelect: 'none',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'center',
              borderColor: 'hsl(var(--n) / var(--tw-border-opacity))',
              textAlign: 'center',
              transitionProperty:
                'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
              transitionDuration: '200ms',
              transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
              borderRadius: 'var(--rounded-btn, 0.5rem)',
              height: '3rem',
              paddingLeft: '1rem',
              paddingRight: '1rem',
              fontSize: '0.875rem',
              lineHeight: '1em',
              minHeight: '3rem',
              fontWeight: '600',
              textTransform: 'var(--btn-text-case, uppercase)',
              textDecorationLine: 'none',
              borderWidth: 'var(--border-btn, 1px)',
              animation: 'button-pop var(--animation-btn, 0.25s) ease-out',
              '--tw-border-opacity': '1',
              '--tw-bg-opacity': '1',
              backgroundColor: 'hsl(var(--n) / var(--tw-bg-opacity))',
              '--tw-text-opacity': '1',
              color: 'hsl(var(--nc) / var(--tw-text-opacity))',
              '&.loading': {
                pointerEvents: 'none',
              },
              '&.loading:hover': {
                pointerEvents: 'none',
              },
              '&.loading:before': {
                marginRight: '0.5rem',
                height: '1rem',
                width: '1rem',
                borderRadius: '9999px',
                borderWidth: '2px',
                animation: 'spin 2s linear infinite',
                content: '""',
                borderTopColor: 'transparent',
                borderLeftColor: 'transparent',
                borderBottomColor: 'currentColor',
                borderRightColor: 'currentColor',
              },
              ':active:hover': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':active:focus': {
                animation: 'none',
                transform: 'scale(var(--btn-focus-scale, 0.95))',
              },
              ':hover': {
                '--tw-border-opacity': '1',
                borderColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor:
                  'hsl(var(--nf, var(--n)) / var(--tw-bg-opacity))',
              },
              ':focus-visible': {
                outline: '2px solid hsl(var(--nf))',
                outlineOffset: '2px',
              },
              '&.glass:hover': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass.btn-active': {
                '--glass-opacity': '25%',
                '--glass-border-opacity': '15%',
              },
              '&.glass:focus-visible': {
                outline: '2px solid 0 0 2px currentColor',
              },
              '&.loading.btn-square:before': {
                marginRight: '0px',
              },
              '&.loading.btn-circle:before': {
                marginRight: '0px',
              },
              '&.loading.btn-xl:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-lg:before': {
                height: '1.25rem',
                width: '1.25rem',
              },
              '&.loading.btn-sm:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '&.loading.btn-xs:before': {
                height: '0.75rem',
                width: '0.75rem',
              },
              '.btn-group > input[type="radio"]:checked&': {
                '--tw-border-opacity': '1',
                borderColor: 'hsl(var(--p) / var(--tw-border-opacity))',
                '--tw-bg-opacity': '1',
                backgroundColor: 'hsl(var(--p) / var(--tw-bg-opacity))',
                '--tw-text-opacity': '1',
                color: 'hsl(var(--pc) / var(--tw-text-opacity))',
              },
              '.btn-group > input[type="radio"]:checked&:focus-visible': {
                outline: '2px solid hsl(var(--p))',
              },
              '.btn-group > &:not(:first-of-type)': {
                marginLeft: '-1px',
                borderTopLeftRadius: '0px',
                borderBottomLeftRadius: '0px',
              },
              '.btn-group > &:not(:last-of-type)': {
                borderTopRightRadius: '0px',
                borderBottomRightRadius: '0px',
              },
              '@media (prefers-reduced-motion: reduce)': {
                '&.loading:before': {
                  animation: 'spin 10s linear infinite',
                },
              },
            }}
          >
            Yay!d
          </label>
        </div>
      </div>
    </div>
  </>
)


