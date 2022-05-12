import { writable, type Writable } from "svelte/store";

export const navbarElement: Writable<undefined | { icon: unknown, name: string, path: string }> = writable(undefined)