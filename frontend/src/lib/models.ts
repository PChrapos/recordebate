import { readdirSync, lstatSync, existsSync, unlinkSync, renameSync } from 'fs'
import path from 'path'
import { getVideoDurationInSeconds } from 'get-video-duration'

const dataFolder = '../data'
export const getRecordedModels = () => {
    if (!existsSync(dataFolder)) return []
    return readdirSync(dataFolder).filter(model => lstatSync(path.join(dataFolder, model)).isDirectory())
}

export const getVideos = (model: string) => {
    if (!getRecordedModels().includes(model)) return []
    return readdirSync(path.join(dataFolder, model)).filter(video => lstatSync(path.join(dataFolder, model, video)).isFile() && video.endsWith('.mp4') && !video.endsWith('.tmp.mp4'))
}

export const deleteVideo = (model: string, video: string) => {
    if (!getVideos(model).includes(video)) return false
    const videoPath = path.join(dataFolder, model, video)
    unlinkSync(videoPath)
    const thumbnailPath = videoPath.substring(0, videoPath.length - 3) + "png"
    if (existsSync(thumbnailPath)) unlinkSync(thumbnailPath)
    return true
}

export const renameVideo = (model: string, video: string, newName: string) => {
    if (!getVideos(model).includes(video)) return false
    if (!newName.endsWith('.mp4') || newName.endsWith('.tmp.mp4')) return false
    const videoPath = path.join(dataFolder, model, video)
    const newVideoPath = path.join(dataFolder, model, newName)
    renameSync(videoPath, newVideoPath)
    const baseName = video.substring(0, video.length - 4)
    const newBaseName = newName.substring(0, newName.length - 4)
    const thumbnailPath = path.join(dataFolder, model, baseName + ".png")
    const newThumbnailPath = path.join(dataFolder, model, newBaseName + ".png")
    if (existsSync(thumbnailPath)) renameSync(thumbnailPath, newThumbnailPath)
    return true
}

export const getThumbnails = (model: string) => {
    if (!getRecordedModels().includes(model)) return []
    return readdirSync(path.join(dataFolder, model)).filter(video => lstatSync(path.join(dataFolder, model, video)).isFile() && video.endsWith('.png'))
}

export const getVideoDuration = async (model: string, video: string) => {
    return await getVideoDurationInSeconds(path.join(dataFolder, model, video))
}