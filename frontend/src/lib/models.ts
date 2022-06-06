import { existsSync, lstatSync, readdirSync, renameSync, unlinkSync } from 'fs'
import { getVideoDurationInSeconds } from 'get-video-duration'
import path from 'path'
import { redisClient } from './redis'

const dataFolder = '../data'
export const getRecordedModels = async () => {
    const cacheModels = await redisClient.get('recordedModels')
    if (cacheModels) return JSON.parse(cacheModels) as string[]
    if (!existsSync(dataFolder)) return []
    const models = readdirSync(dataFolder).filter(model => lstatSync(path.join(dataFolder, model)).isDirectory())
    await redisClient.set('recordedModels', JSON.stringify(models))
    return models
}

export const getVideos = async (model: string) => {
    const qualifiedName = `videos.${model}`
    const cache = await redisClient.get(qualifiedName)
    if (cache) return JSON.parse(cache) as string[]
    if (!(await getRecordedModels()).includes(model)) return []
    const videos = readdirSync(path.join(dataFolder, model)).filter(video => lstatSync(path.join(dataFolder, model, video)).isFile() && video.endsWith('.mp4') && !video.endsWith('.tmp.mp4'))
    await redisClient.set(qualifiedName, JSON.stringify(videos))
    return videos
}

export const deleteVideo = async (model: string, video: string) => {
    if (!(await getVideos(model)).includes(video)) return false
    const videoPath = path.join(dataFolder, model, video)
    unlinkSync(videoPath)
    const thumbnailPath = videoPath.substring(0, videoPath.length - 3) + "png"
    if (existsSync(thumbnailPath)) unlinkSync(thumbnailPath)
    await redisClient.del(`videos.${model}`)
    await redisClient.del(`videoDuration.${model}.${video}`)
    return true
}

export const renameVideo = async (model: string, video: string, newName: string) => {
    if (!(await getVideos(model)).includes(video)) return false
    if (!newName.endsWith('.mp4') || newName.endsWith('.tmp.mp4')) return false
    const videoPath = path.join(dataFolder, model, video)
    const newVideoPath = path.join(dataFolder, model, newName)
    renameSync(videoPath, newVideoPath)
    const baseName = video.substring(0, video.length - 4)
    const newBaseName = newName.substring(0, newName.length - 4)
    const thumbnailPath = path.join(dataFolder, model, baseName + ".png")
    const newThumbnailPath = path.join(dataFolder, model, newBaseName + ".png")
    if (existsSync(thumbnailPath)) renameSync(thumbnailPath, newThumbnailPath)
    await redisClient.del(`videos.${model}`)
    await redisClient.del(`videoDuration.${model}.${video}`)
    return true
}

export const getThumbnails = async (model: string) => {
    const qualifiedName = `thumbnails.${model}`
    const cache = await redisClient.get(qualifiedName)
    if (cache) return JSON.parse(cache) as string[]
    if (!(await getRecordedModels()).includes(model)) return []
    const thumbnails = readdirSync(path.join(dataFolder, model)).filter(video => lstatSync(path.join(dataFolder, model, video)).isFile() && video.endsWith('.png'))
    redisClient.set(qualifiedName, JSON.stringify(thumbnails))
    return thumbnails
}

export const getVideoDuration = async (model: string, video: string) => {
    const qualifiedName = `videoDuration.${model}.${video}`
    const cache = await redisClient.get(qualifiedName)
    if (cache) return Number.parseInt(cache)
    const duration = await getVideoDurationInSeconds(path.join(dataFolder, model, video))
    redisClient.set(qualifiedName, duration)
    return duration
}