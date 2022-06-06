import { getRecordedModels } from "$lib/models";
import type { RequestEvent } from "@sveltejs/kit/types/private";
import { openSync, readdirSync, readSync, statSync, closeSync } from 'fs';
import path from 'path';

const CHUNK_SIZE = 1024 * 1024

export const get = async ({ request, params, url }: RequestEvent) => {
    const model = params.name
    const video = url.searchParams.get("video")
    const videos = readdirSync(path.join('../data', model)).filter(file => file.endsWith(".mp4"))
    if (!model || !(await getRecordedModels()).includes(model) || !video || !videos.includes(video)) return {
        status: 400
    }
    const videoFile = path.join('../data', model, video)
    const { size } = statSync(videoFile)

    let rangeStart = 0
    let rangeEnd = size
    const range = request.headers.get('range')
    if (range) {
        const [reqStart, reqEnd] = range.replace('bytes=', '').trim().split('-')
        rangeStart = Math.min(Number.parseInt(reqStart), size - 1)
        if (reqEnd) {
            rangeEnd = Math.min(rangeEnd, Number.parseInt(reqEnd))
        }
    }
    const sendSize = Math.min(CHUNK_SIZE, rangeEnd - rangeStart)

    const headers = new Headers()
    headers.set('accept-ranges', 'bytes')
    headers.set('access-control-expose-headers', 'Content-Length,Content-Range,ETag,Server-Timing,X-Content-Type-Options')
    headers.set('content-type', 'video/mp4')
    headers.set('content-length', sendSize.toString())
    headers.set('content-range', `bytes ${rangeStart}-${rangeStart + sendSize}/${size}`)

    const buffer = Buffer.alloc(sendSize)
    const fd = openSync(videoFile, 'r')
    const nread = readSync(fd, buffer, { position: rangeStart, length: sendSize })
    closeSync(fd)
    return {
        status: 206,
        headers,
        body: Uint8Array.from(buffer.slice(0, nread))
    }
}
