export const formatTitle = (title: string) =>
  title.replace(/^[-_]*(.)/, (_, c) => c.toUpperCase())
    .replace(/[-_]+(.)/g, (_, c) => ' ' + c.toUpperCase())

export const firstUpper = (str: string) => str[0].toUpperCase() + str.slice(1)
