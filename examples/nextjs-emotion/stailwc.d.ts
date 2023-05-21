import type { CSSInterpolation } from '@emotion/serialize'

export type TemplateFn<R> = (
  strings: Readonly<TemplateStringsArray>,
  ...values: readonly string[]
) => R

export type TwFn = TemplateFn<CSSInterpolation>

export type TwComponentMap = {
  [K in keyof JSX.IntrinsicElements]: TemplateFn<TwComponent<K>>
}

export type TwComponent<K extends keyof JSX.IntrinsicElements> = (
  props: JSX.IntrinsicElements[K]
) => JSX.Element

type TwComponentWrapper = <T extends ComponentType<any>>(
  component: T
) => TemplateFn<T>

declare module 'react' {
  interface DOMAttributes<T> {
    tw?: string
    css?: CSSInterpolation
  }
}

declare global {
  const tw: TwFn & TwComponentMap & TwComponentWrapper

  namespace JSX {
    interface IntrinsicAttributes extends React.Attributes {
      tw?: string
    }
  }
}
