

var vel_n_frame = 0;

const CanvasVelocityController = class {
    constructor(canvas, vel_max, rot_max) {
        this.canvas = canvas;
        this.touched_point = null;
        this.vel_range = vel_max;
        this.rot_range = rot_max;
    }

    setup() {
        // 共通処理をまとめとく。
        const point = (e) => {
            const cr = this.canvas.getBoundingClientRect();
            return [e.clientX - cr.left - cr.width/2, e.clientY - cr.top - cr.height/2]
        };
        
        this.canvas.addEventListener('touchstart', (e) => {
            this.touched_point = point(e.changedTouches[0]);
        });
        this.canvas.addEventListener('mousedown', (e) => {
            this.touched_point = point(e);
        });
        this.canvas.addEventListener('touchmove', (e) => {
            if (this.touched_point !== null)
                this.touched_point = point(e.changedTouches[0]);
        });
        this.canvas.addEventListener('mousemove', (e) => {
            if (this.touched_point !== null)
                this.touched_point = point(e);
        });
        this.canvas.addEventListener('touchend', (e) => {
            this.touched_point = null;
        });
        this.canvas.addEventListener('mouseup', (e) => {
            this.touched_point = null;
        });

    }

    update() {
        let canvas = this.canvas;
        let ctx = this.canvas.getContext('2d');
        // clear
        ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);  
        
        // text
        ctx.strokeStyle = 'blue';  // 線の色
        ctx.beginPath();
        ctx.strokeText('Frame Number:' + vel_n_frame++, 10, 30);
        ctx.stroke();

        // draw axis
        ctx.strokeStyle = 'blue';  // 線の色
        ctx.beginPath();
        ctx.moveTo(0, canvas.width/2);
        ctx.lineTo(canvas.height, canvas.width/2);
        ctx.moveTo(canvas.height/2, 0);
        ctx.lineTo(canvas.height/2, canvas.width);
        ctx.stroke();

        let point = this.touched_point;
        if (this.touched_point === null) {
            point = [0, 0];
        }


        ctx.strokeStyle = 'black';
        ctx.fillStyle = 'red';
        ctx.beginPath();
        ctx.moveTo(canvas.width/2, canvas.height/2);
        ctx.lineTo(point[0] + canvas.width/2, point[1]+ canvas.height/2);
        ctx.stroke();

        ctx.strokeStyle = 'black';
        ctx.fillStyle = 'cyan';
        ctx.beginPath();
        ctx.arc(point[0] + canvas.width/2, point[1]+ canvas.height/2, 30, 0, Math.PI*2, true);
        ctx.stroke();
        ctx.fill();

        const vel_range = this.vel_range;;
        const rot_range = this.rot_range;
        const vx = -point[1] / canvas.height/2 * vel_range;
        const wz = -point[0] / canvas.width/2 * rot_range;
        return [vx, wz];
    }
}

function init_robot_function_velocity(robot) {
      
    const canvas = document.getElementById('velocity_canvas');
    const point = (e) => {
      const cr = canvas.getBoundingClientRect();
      return [e.clientX - cr.left - cr.width/2, e.clientY - cr.top - cr.height/2] };

      
    canvas.addEventListener('touchstart', (e) => {
        touched_point = point(e.changedTouches[0]);
    });

    canvas.addEventListener('mousedown', (e) => {
        touched_point = point(e);
    });

    canvas.addEventListener('touchmove', (e) => {
        if (touched_point !== null)
            touched_point = point(e.changedTouches[0]);
    });
    canvas.addEventListener('mousemove', (e) => {
        if (touched_point !== null)
            touched_point = point(e);
    });
    canvas.addEventListener('touchend', (e) => {
        touched_point = null;
    });
    canvas.addEventListener('mouseup', (e) => {
        touched_point = null;
    });

    setInterval(()=>{update_velocity_canvas(robot)}, 100);
}


var _locked = false;
function toggleScroll() {
  if (_locked) {
    _locked = false;
    unlockScroll();
  } else {
    _locked = true;
    lockScroll();
  }
}


function handleMouseWheel(e) {
  e.preventDefault();
}

function handleTouchMove(e) {
  e.preventDefault();
}

function handleKeyDown(e) {
  switch (e.keyCode) {
    case 0x25:
    case 0x26:
    case 0x27:
    case 0x28:
      e.preventDefault();
      break;
  }
}
function lockScroll() {
  document.addEventListener("mousewheel", handleMouseWheel, { passive: false });
  document.addEventListener("touchmove", handleTouchMove, { passive: false });
  document.addEventListener("keydown", handleKeyDown, { passive: false });
  document.body.style.overflow = "hidden";
}

function unlockScroll() {
  document.removeEventListener("mousewheel", handleMouseWheel, { passive: false });
  document.removeEventListener("touchmove", handleTouchMove, { passive: false });
  document.removeEventListener("keydown", handleKeyDown, { passive: false });
  document.body.style.overflow = "visible";
}


function update_velocity_canvas(robot) {
  const canvas = document.getElementById('velocity_canvas');
  let ctx = canvas.getContext('2d');
  // clear
  ctx.clearRect(0, 0, canvas.width, canvas.height);  
  if (!_locked) {
    return;
  }

  // text
  ctx.strokeStyle = 'blue';  // 線の色
  ctx.beginPath();
  ctx.strokeText('Frame Number:' + vel_n_frame++, 10, 30);
  ctx.stroke();

  // draw axis
  ctx.strokeStyle = 'blue';  // 線の色
  ctx.beginPath();
  ctx.moveTo(0, canvas.width/2);
  ctx.lineTo(canvas.height, canvas.width/2);
  ctx.moveTo(canvas.height/2, 0);
  ctx.lineTo(canvas.height/2, canvas.width);
  ctx.stroke();

  let point = touched_point;
  if (touched_point === null) {
    point = [0, 0];
  }


  ctx.strokeStyle = 'black';
  ctx.fillStyle = 'red';
  ctx.beginPath();
  ctx.moveTo(canvas.width/2, canvas.height/2);
  ctx.lineTo(point[0] + canvas.width/2, point[1]+ canvas.height/2);
  ctx.stroke();

  ctx.strokeStyle = 'black';
  ctx.fillStyle = 'cyan';
  ctx.beginPath();
  ctx.arc(point[0] + canvas.width/2, point[1]+ canvas.height/2, 30, 0, Math.PI*2, true);
  ctx.stroke();
  ctx.fill();

  const vel_range = document.getElementById('form_velocity_control_range');
  const rot_range = document.getElementById('form_rotation_control_range');
  const vx = -point[1] / canvas.height/2 * vel_range.value;
  const wz = -point[0] / canvas.width/2 * rot_range.value;

  robot.set_velocity(vx, wz);
}
