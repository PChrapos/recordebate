import { getThumbnailData, getThumbnails } from "$lib/models";
import type { RequestHandler } from "@sveltejs/kit";

export const get: RequestHandler<{ name: string }> = async ({ params, url }) => {
    const model = params.name
    if (!model) return {
        status: 400
    }

    const images = await getThumbnails(model)
    const video = url.searchParams.get('video')
    const sizeParam = url.searchParams.get('size')
    const image = images.find(image => {
        return image.substring(0, image.length - 4) == video?.replace('.mp4', '')
    }) ?? images.at(-1)

    if (!image) {
        return {
            status: 302,
            headers: { Location: `https://roomimg.stream.highwebmedia.com/riw/${model}.jpg` }
        }
    }
    const size = sizeParam ? { width: Number.parseInt(sizeParam.split('x')[0]), height: Number.parseInt(sizeParam.split('x')[1]) } : undefined
    const data = await getThumbnailData(model, image, size)

    return {
        headers: {
            'Content-type': 'image/png',
            'Cache-Control': 'public, max-age=36000'
        },
        body: Uint8Array.from(data)
    }
}