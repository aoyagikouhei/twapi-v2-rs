import { Tweets } from "./responses/tweets"
import * as fs from 'fs'
import {objectToCamel} from 'ts-case-convert'

const data = fs.readFileSync('./data.json', 'utf-8')
const data2: Tweets = objectToCamel(JSON.parse(data)) as any
console.log(data2)
console.log(data2.contextAnnotations[0].domain)