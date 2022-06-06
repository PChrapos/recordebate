import { getRecordedModels } from '$lib/models'

export const get = async () => {
    return {
        body: { models: await getRecordedModels() }
    }
}