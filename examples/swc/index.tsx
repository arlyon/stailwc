import { TailwindStyle } from "stailwc";

const Button = tw.button`content-[Hello World] px-4 py-2 bg-red-500 rounded-full text-white`;

export const App = () => (
  <div tw="w-screen h-screen">
    <TailwindStyle />
    <Button />
  </div>
);
