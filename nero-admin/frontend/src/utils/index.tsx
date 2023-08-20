export function formatThing(thing: any): string {
    return Object.values(thing.id)[0] as string
}