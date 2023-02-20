import _styled from "@emotion/styled";
_styled;

const UnderlineLink = tw.a`underline`;

export default function App() {
  return (
    <div tw="mx-auto my-16 w-max">
      <main tw="flex flex-col gap-4">
        <div tw="text-center">
          <h1 tw="font-extrabold text-xl">
            Welcome to{" "}
            <UnderlineLink href="https://reactjs.org">React</UnderlineLink> and{" "}
            <UnderlineLink href="https://stailwc.vercel.app">
              stailwc
            </UnderlineLink>
            !
          </h1>
          <p tw="text-neutral-500">
            Get started by editing <code>App.tsx</code>
          </p>
        </div>

        <a href="https://tailwindcss.com/docs/utility-first">
          <div tw="bg-gradient-to-br text-white from-teal-400 to-blue-700 rounded border border-teal-200 p-4">
            <h2 tw="text-lg font-bold">tailwind.css &rarr;</h2>
            <p>Documentation for the types of classes available.</p>
          </div>
        </a>

        <a href="https://reactjs.org/docs">
          <div tw="bg-neutral-100 rounded border p-4">
            <h2 tw="text-lg font-bold">Documentation &rarr;</h2>
            <p>Find in-depth information about React features and API.</p>
          </div>
        </a>

        <a href="https://reactjs.org/learn">
          <div tw="bg-neutral-100 rounded border p-4">
            <h2 tw="text-lg font-bold">Learn &rarr;</h2>
            <p>Learn about React in an interactive tutorial!</p>
          </div>
        </a>

        <a href="https://vitejs.dev">
          <div tw="bg-neutral-100 rounded border p-4">
            <h2 tw="text-lg font-bold">Vite &rarr;</h2>
            <p>Discover the Vite bundler.</p>
          </div>
        </a>
      </main>

      <footer tw="flex flex-row items-center py-8 justify-center gap-4">
        <a
          href="https://vitejs.dev/guide"
          target="_blank"
          rel="noopener noreferrer"
        >
          Powered by{" "}
          <span>
            <img src="/vite.svg" alt="Vite Logo" width={72} height={16} />
          </span>
        </a>
        +
        <a href="https://github.com/arlyon/stailwc" tw="font-mono">
          arlyon/stailwc
        </a>
      </footer>
      <div tw="flex flex-row justify-center items-center gap-3">
        Consider becoming a{" "}
        <a
          href="https://github.com/sponsors/arlyon"
          tw="flex flex-row bg-gray-50 border shadow-sm text-pink-500 border-pink-300 rounded justify-center items-center w-min px-3 py-2"
          target="_top"
          aria-label="Sponsor @arlyon"
        >
          <svg
            aria-hidden="true"
            viewBox="0 0 16 16"
            fill="currentColor"
            tw="w-4 h-4 mr-2 transition-colors"
          >
            <path
              fillRule="evenodd"
              d="M4.25 2.5c-1.336 0-2.75 1.164-2.75 3 0 2.15 1.58 4.144 3.365 5.682A20.565 20.565 0 008 13.393a20.561 20.561 0 003.135-2.211C12.92 9.644 14.5 7.65 14.5 5.5c0-1.836-1.414-3-2.75-3-1.373 0-2.609.986-3.029 2.456a.75.75 0 01-1.442 0C6.859 3.486 5.623 2.5 4.25 2.5zM8 14.25l-.345.666-.002-.001-.006-.003-.018-.01a7.643 7.643 0 01-.31-.17 22.075 22.075 0 01-3.434-2.414C2.045 10.731 0 8.35 0 5.5 0 2.836 2.086 1 4.25 1 5.797 1 7.153 1.802 8 3.02 8.847 1.802 10.203 1 11.75 1 13.914 1 16 2.836 16 5.5c0 2.85-2.045 5.231-3.885 6.818a22.08 22.08 0 01-3.744 2.584l-.018.01-.006.003h-.002L8 14.25zm0 0l.345.666a.752.752 0 01-.69 0L8 14.25z"
            />
          </svg>
          <span>Sponsor</span>
        </a>
      </div>
    </div>
  );
}
