import { getRecordedModels, getThumbnails } from "$lib/models";
import type { RequestHandler } from "@sveltejs/kit";
import { readFileSync } from 'fs';
import path from 'path';

export const get: RequestHandler<{ name: string }> = async ({ params, url }) => {
    const model = params.name
    if (!model) return {
        status: 400
    }
    const images = getThumbnails(model)
    const video = url.searchParams.get("video")
    const image = images.find(image => {
        return image.substring(0, image.length - 4) == video?.replace('.mp4', '')
    }) ?? images.at(-1)

    if (!image) {
        return {
            status: 302,
            headers: { Location: `https://roomimg.stream.highwebmedia.com/riw/${model}.jpg` }
        }
    }
    const data = readFileSync(path.join('../data', model, image));

    return {
        headers: {
            "Content-type": 'image/png'
        },
        body: Uint8Array.from(data)
    }
}