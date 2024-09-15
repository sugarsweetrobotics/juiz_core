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
    constructor(turtleSim, index) {
        this.index = index;
        this.turtleSim = turtleSim;
    }

    async setup() {
        //this.container_processes = await this.container.processes();
        this.get_pose_process = await this.turtleSim.turtle_sim_cont.process({'type_name': 'get_turtle_pose'});
        if (this.get_pose_process === null) {
            console.error('Turtle(index=', this.index, ').setup() failed. Can not find turtle_sim::get_turtle_pose process');
        }
        this.set_target_velocity_process = await this.turtleSim.turtle_sim_cont.process({'type_name': 'set_turtle_target_velocity'});
        if (this.set_target_velocity_process === null) {
            console.error('Turtle(index=', this.index, ').setup() failed. Can not find turtle_sim::set_turtle_target_velocity process');
        }
        this.get_lidar_process = await this.turtleSim.turtle_sim_cont.process({'type_name': 'get_turtle_lidar'});
        if (this.set_target_velocity_process === null) {
            console.error('Turtle(index=', this.index, ').setup() failed. Can not find turtle_sim::get_turtle_lidar process');
        }
        return this;
    }

    async get_pose() {
        if (this.get_pose_process === null) {
            console.error('Turtle.get_pose() failed. get_pose_process is null');
            return null;
        }
        let value = await this.get_pose_process.call_process({'index': this.index});
        // console.log('value:', value);
        this.pose = new Pose(value[0], value[1][0], value[1][1], value[1][2]);
        return this.pose;
    }
    async get_lidar() {
        if (this.get_lidar_process === null) {
            console.error('Turtle.get_lidar() failed. get_lidar_process is null');
            return null;
        }
        let value = await this.get_lidar_process.call_process({'index': this.index});
        // console.log('value:', value);
        // this.pose = new Pose(value[0], value[1][0], value[1][1], value[1][2]);
        //return this.pose;
        return value;
    }

    async set_target_velocity(vel) {
        // console.log('turtle(', this.index, ').set_target_velocity(', vel, ') called');
        if (this.set_velocity_process === null) {
            console.error('Turtle.set_target_velocity() failed. set_target_velocity_process is null');
            return null;
        }
        let args = {
            "index": this.index,
            "velocity": {
                "vx": vel.vx,
                "vy": vel.vy,
                "wz": vel.wz,
            }
        };
        let value = await this.set_target_velocity_process.call_process(args);
    }
}

const TurtleSimDrawer = class {
    constructor(canvas_elem, width_m, height_m, pos_topleft_x_m, pos_topleft_y_m) {
        this.canvas = canvas_elem;
        this.width = this.canvas.width;
        this.height = this.canvas.height;
        this.width_m = width_m;
        this.height_m = height_m;
        this.position_of_topleft = {
            x: pos_topleft_x_m,
            y: pos_topleft_y_m
        };
        this.scale = {
            x: width_m / this.width,
            y: height_m  / this.height,
        }; // [m / px]

        if (this.scale.x !== this.scale.y) {
            console.error('現在、x-yのスケールが違うマップに対応していません。面倒なので。')
        }
        this.center = {
            x: this.width / 2,
            y: this.height / 2
        };
        this.origin_px = {
            x: -this.position_of_topleft.x / this.scale.x,
            y: this.position_of_topleft.y / this.scale.y
        };
    }

    pose_to_px(pose) {
        console.log('pose_to_px(', pose, ') called');
        const retval =  {
            x: this.origin_px.x + pose[0] / this.scale.x,
            y: this.origin_px.y - pose[1] / this.scale.y,
            th: pose[2]
        };
        //console.log(' - returns: ', retval);
        return retval;
    }

    draw(turtleSim) {
        let ctx = this.canvas.getContext('2d');
        ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        //ctx.fillStyle = '#8F8F8F';
        //ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
        for(let turtle of turtleSim.profile.turtles) {
            let pose_px = this.pose_to_px(turtle.pose);
            let lidar_data = turtle.lidar;
            this.draw_pose(ctx, pose_px, lidar_data);
        }
    }

    draw_turtle(ctx, turtle) {

    }

    draw_pose(ctx, pose_px, lidar_data) {

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

        this.draw_lidar(ctx, lidar_data)
        ctx.rotate(th);
        ctx.translate(-x, -y);
    }

    draw_lidar(ctx, lidar_data) {
        // console.log('draw_lidar(', lidar_data, ')');
        // ctx.rotate(-lidar_data.min_angle);
        
        const scale =4.0;
        for (let i = 0;i < lidar_data.ranges.length; ++i) {
            const range = lidar_data.ranges[i];
            const angle = lidar_data.min_angle + i * lidar_data.angle_res;
            const lidar_end_pos_m = {
                x: range * Math.cos(angle),
                y: -range * Math.sin(angle)
            }


            ctx.moveTo(0, 0);
            ctx.lineTo(lidar_end_pos_m.x / this.scale.x, lidar_end_pos_m.y / this.scale.y);
            // ctx.rotate(-lidar_data.angle_res);
        }

        ctx.stroke();
        // ctx.rotate((lidar_data.max_angle + lidar_data.angle_res))
    }
}

const TurtleSim = class {
    constructor() { /* コンストラクタ */
        this.system = new juiz.System();
        this.turtles = [];
        this.initialized = false;
    }


    async setup() {
        this.turtles = [];
        console.log('TurtleSim.setup() called');
        const prof = await this.system.setup();
        console.log('System(prof=', prof, ')');
        const containers = await this.system.containers();
        this.turtle_sim_cont = await this.system.container({'type_name': 'turtle_sim'});
        this.get_profile_proc = await this.turtle_sim_cont.process({'type_name': 'get_profile'});
        this.get_map_proc = await this.turtle_sim_cont.process({'type_name': 'get_map'});
        this.get_map_metadata_proc = await this.turtle_sim_cont.process({'type_name': 'get_map_metadata'});
        this.initialized = true;
        this.profile = await this.get_profile();
        this.turtles = await Promise.all(this.profile.turtles.map(async (t, i) => {return await (new Turtle(this, i)).setup();}));
        this.map_metadata = await this.get_map_metadata();
        // console.log('metadata: ', this.map_metadata);
        return this;
    }

    async get_map() {
        return await this.get_map_proc.call_process({});
    }

    async get_map_metadata() {
        return await this.get_map_metadata_proc.call_process({});
    }

    async get_profile() {
        const prof = await this.get_profile_proc.call_process({});
        // console.log('prof:', prof);
        return prof;
    }


    async loop() {
        if (!this.initialized) {
            return false;
        }

        this.profile = await this.get_profile()
        return true;
    }
  }