import type { RequestHandler } from '@sveltejs/kit'
import { existsSync, readFileSync, writeFileSync } from 'fs'

const configFile = '../config/config.json'
interface Config {
    trackedModels: string[]
}

const getConfig = (): Config => {
    if (!existsSync(configFile)) {
        return {
            trackedModels: []
        }
    }
    return JSON.parse(readFileSync(configFile, 'utf-8'))
}

const writeConfig = (config: Config) => {
    writeFileSync(configFile, JSON.stringify(config))
}

export const get = async () => {
    return {
        body: {
            trackedModels: getConfig()['trackedModels'] ?? []
        }
    }
}

export const put: RequestHandler = async ({ request }) => {
    const params = await request.json()
    if (!params.name) return {}
    const config = getConfig()
    config.trackedModels = config.trackedModels.filter(model => model != params.name)
    config.trackedModels.push(params.name)
    writeConfig(config)
    return {
        body: {
            msg: `added model ${params.name}`
        }
    }
}

export const del: RequestHandler = async ({ request }) => {
    const params = await request.json()
    const config = getConfig()
    config.trackedModels = config.trackedModels.filter(model => model != params.name)
    writeConfig(config)
    return {
        body: {
            msg: `deleted model ${params.name}`
        }
    }
}