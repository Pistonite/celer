module.exports = {
    js: {
        filePatterns: [
            /\.ts$/,
            /\.tsx$/,
        ],
        privatePatterns: [
            /^const/,
            /^type/,
            /^interface/,
            /^class/,
            /^function/,
            /^async function/,
        ],
        publicPatterns: [
            /^export const/,
            /^export type/,
            /^export interface/,
            /^export class/,
            /^export function/,
            /^export async function/,
        ],
        allowInBetweenPatterns: []
    },
    rs: {
        filePatterns: [
            /\.rs$/,
        ],
        privatePatterns: [
            /^struct/,
            /^enum/,
            /^type/,
            /^fn/,
            /^async fn/,
            /^impl/,
            /^trait/,
            /^macro_rules/,
        ],
        publicPatterns: [
            /^pub struct/,
            /^pub enum/,
            /^pub type/,
            /^pub fn/,
            /^pub async fn/,
            /^pub trait/,
            /^pub macro_rules/,
        ],
        allowInBetweenPatterns: [
            /^#/,
        ]
    }
}