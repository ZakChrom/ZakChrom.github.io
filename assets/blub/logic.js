const {
    BACKWARD, FORWARD, LEFT, RIGHT, UP, DOWN, 
    sound, BEAT, BREAK, 
    cell, texture, 
    DIRECTIONAL, TICK
} = CELLMODDING

//blob:AAAGAAAAAGQBAUgBA0gBBGgBBUgBEEgB///7AAAADFwBDUgBG1QBHEgBH1QBO1ABPEgBPjw/SAFLVAFMSAEAAAz///CwSAGzVAG0SAHAYAHBSAHEbAHFSAHQSAHTVAHUSAHwSAHzVQH///v///C7VAG8SAG/VAHMWAHNSAHbVAHcSAHfVAH7VAH8SAH/VAE=
const textures = texture('https://calion.dev/assets/blub/logic.png')

cell({
    name: '[Logic] Button',
    tx: 3, ty: 0,
    update: TICK,
    atlas: textures,
    
    clicked() {
        const c = this.get(this.dir)
        if(c)c.data++
    }
})
// cell({
//     name: '[Logic] Switch',
//     tx: 3, ty: 2,
//     update: TICK,
//     atlas: textures,
    
//     clicked() {
//         if(this.data==0)this.data==1
//         else{this.data==0}
//         const c = this.get(this.dir)
//         if(c)c.data=this.data
//     }
// })
cell({
    name: '[Logic] Random',
    tx: 3, ty: 1,
    update: TICK,
    atlas: textures,
    
    tick() {
        const c = this.get(this.dir)
        if(c)c.data=Math.floor(random() * 100)
    }
})
cell({
    name: '[Logic] And',
    tx: 0, ty: 0,
    update: TICK,
    atlas: textures,
    
    tick() {
        const i1 = this.look(LEFT)
        const i2 = this.look(RIGHT)
        const o = this.look(UP)
        let a,b = 0
        if(i1)a=i1.data
        if(i2)b=i2.data
        if(o)o.data=a&b
    }
})
cell({
    name: '[Logic] Nand',
    tx: 0, ty: 1,
    update: TICK,
    atlas: textures,
    
    tick() {
        const i1 = this.look(LEFT)
        const i2 = this.look(RIGHT)
        const o = this.look(UP)
        let a,b = 0
        if(i1)a=i1.data
        if(i2)b=i2.data
        if(o)o.data=~(a & b)
    }
})
cell({
    name: '[Logic] Or',
    tx: 1, ty: 0,
    update: TICK,
    atlas: textures,
    
    tick() {
        const i1 = this.look(LEFT)
        const i2 = this.look(RIGHT)
        const o = this.look(UP)
        let a,b = 0
        if(i1)a=i1.data
        if(i2)b=i2.data
        if(o)o.data=a|b
    }
})
cell({
    name: '[Logic] Nor',
    tx: 1, ty: 1,
    update: TICK,
    atlas: textures,
    
    tick() {
        const i1 = this.look(LEFT)
        const i2 = this.look(RIGHT)
        const o = this.look(UP)
        let a,b = 0
        if(i1)a=i1.data
        if(i2)b=i2.data
        if(o)o.data=~(a | b)
    }
})
cell({
    name: '[Logic] Not',
    tx: 0, ty: 2,
    update: TICK,
    atlas: textures,
    
    tick() {
        const i = this.look(DOWN)
        const o = this.look(UP)
        let a = 0
        if(i)a=i.data
        if(o)o.data= ~a
    }
})
cell({
    name: '[Logic] Xor',
    tx: 1, ty: 2,
    update: TICK,
    atlas: textures,
    
    tick() {
        const i1 = this.look(LEFT)
        const i2 = this.look(RIGHT)
        const o = this.look(UP)
        let a,b = 0
        if(i1)a=i1.data
        if(i2)b=i2.data
        if(o)o.data=a^b
    }
})
cell({
    name: '[Logic] Wire',
    tx: 0, ty: 3,
    update: TICK,
    atlas: textures,
    
    tick() {
        const i = this.look(DOWN)
        if(i){this.data=i.data;i.data=0}
    }
})
