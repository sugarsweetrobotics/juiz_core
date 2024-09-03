const Pose = class {
    constructor(ts, x, y, th) {
        this.timestamp = ts === undefined ? 0 : ts;
        this.x = x === undefined ? 0 : x;
        this.y = y  === undefined ? 0 : y;
        this.th = th === undefined ? 0 : th;
    }
}

const Velocity = class {
    constructor(vx, vy, wz) {
        this.vx = vx;
        this.vy = vy;
        this.wz = wz;
    }
}

const Turtle = class {
    constructor(container) {
        this.container = container;
    }

    async setup() {
        this.container_processes = await this.container.processes();
        this.get_pose_process = null;
        this.set_target_velocity_process = null;
        for (let p of this.container_processes) {
            if (p.type_name() === 'get_pose') {
                this.get_pose_process = p;
            } else if (p.type_name() === 'set_target_velocity') {
                this.set_target_velocity_process = p;
            }
        }
    }

    async get_pose() {
        if (this.get_pose_process === null) {
            console.error('Turtle.get_pose() failed. get_pose_process is null');
            return null;
        }
        let value = await this.get_pose_process.call_process();
        this.pose = new Pose(value[0], value[1][0], value[1][1], value[1][2]);
        return this.pose;
    }

    async set_target_velocity(vel) {
        if (this.set_velocity_process === null) {
            console.error('Turtle.set_target_velocity() failed. set_target_velocity_process is null');
            return null;
        }
        let args = {
            "target_velocity": {
                "vx": vel.vx,
                "vy": vel.vy,
                "wz": vel.wz,
            }
        };
        let value = await this.set_target_velocity_process.call_process(args);
    }
}

const TurtleSimDrawer = class {
    constructor(canvas_elem, width_m, height_m) {
        this.canvas = canvas_elem;
        this.width = this.canvas.width;
        this.height = this.canvas.height;
        this.width_m = width_m;
        this.height_m = height_m;
        this.scale = {
            x: width_m / this.width,
            y: height_m  / this.height,
        }; // [m / px]
        this.center = {
            x: this.width / 2,
            y: this.height / 2
        };
    }

    pose_to_px(pose) {
        return {
            x: this.center.x + pose.x / this.scale.x,
            y: this.center.y - pose.y / this.scale.y,
            th: pose.th
        };
    }

    draw(turtleSim) {
        let ctx = this.canvas.getContext('2d');
        ctx.fillStyle = '#8F8F8F';
        ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
        for(let turtle of turtleSim.turtles) {
            let pose_px = this.pose_to_px(turtle.pose);
            this.draw_turtle(ctx, pose_px);
        }
    }

    draw_turtle(ctx, pose_px) {

        ctx.fillStyle = '#00FF00';
        const w = 10;
        const h = 16;
        ctx.beginPath();
        const x = pose_px.x;
        const y = pose_px.y;
        const th = pose_px.th;
        ctx.translate(x, y);
        ctx.rotate(-th);
        ctx.moveTo(-h, -w);
        ctx.lineTo(+0, -w);
        ctx.lineTo(+h, 0);
        ctx.lineTo(+0, +w);
        ctx.lineTo(-h, +w);
        ctx.lineTo(-h, -w);
        ctx.fill();
        ctx.stroke();
        ctx.rotate(th);
        ctx.translate(-x, -y);
    }
}

const TurtleSim = class {
    constructor() { /* コンストラクタ */
        this.system = new System();
        this.turtles = [];
        this.initialized = false;
    }


    async setup() {
        this.turtles = [];
        console.log('TurtleSim.setup() called');
        const prof = await this.system.setup();
        console.log('System(prof=', prof, ')');
        const containers = await this.system.containers();
        for (let c of containers) {
            if (c.type_name() === 'turtle') {
                let t = new Turtle(c);
                await t.setup();
                this.turtles.push(t);
            }
        }
        this.initialized = true;
    }


    async loop() {
        if (!this.initialized) {
            return false;
        }

        for (let t of this.turtles) {
            //await t.set_target_velocity(new Velocity(0.1, 0, 0.05));
            let p = await t.get_pose();
            console.log('pose:', p);
        }
        return true;
    }
  }