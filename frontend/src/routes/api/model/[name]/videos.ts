import { deleteVideo, getRecordedModels, getVideoDuration, getVideos, renameVideo } from "$lib/models";
import type { RequestHandler } from "@sveltejs/kit";

export const get: RequestHandler<{ name: string }> = async ({ params }) => {
    const model = params.name
    if (!model || !getRecordedModels().includes(model)) return {
        status: 400
    }
    const videos = getVideos(model)
    const durations = await Promise.all(videos.map(video => getVideoDuration(model, video)))
    return {
        body: {
            videos: videos.map((video, i) => {
                return {
                    name: video,
                    duration: durations[i]
                }
            })
        }
    }
}

export const del: RequestHandler<{ name: string }> = async ({ request, params: { name } }) => {
    const params = await request.json()
    if (!params.video) return { status: 400 }
    if (!deleteVideo(name, params.video)) return { status: 404 }
    return { status: 200 }
}

export const post: RequestHandler<{ name: string }> = async ({ request, params: { name } }) => {
    const params = await request.json()
    if (!params.video || !params.newName) return { status: 400 }
    if (!renameVideo(name, params.video, params.newName)) return { status: 404 }
    return { status: 200 }
}