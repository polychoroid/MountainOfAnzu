const rust = import('./pkg/mountain_of_anzu');
const canvas = document.getElementById('canvas');
const gl = canvas.getContext('webgl', { antialias: true});
const FPS = 30.0; // Frames per second 
const FPS_THROTTLE = 1000.0 / FPS; // Frame limit to 30


rust.then(m => {
  if(!gl) {
    alert('Failed to initailize WebGL');
    return;
  }
  
  const client = new m.GameClient();

  const intitialTime = Date.now();
  let lastDrawTime = -1;

  function render()
  {
    window.requestAnimationFrame(render);
    const currentTime = Date.now();
    
    if(currentTime >= lastDrawTime + FPS_THROTTLE)
    {
      lastDrawTime = currentTime;

      if(window.innerHeight != canvas.height || window.innerWidth != canvas.width)
      {
        canvas.height = window.innerHeight;
        canvas.clientHeight = window.innerHeight;
        canvas.style.height= window.innerHeight;

        canvas.width = window.innerWidth;
        canvas.clientWidth = window.innerWidth;
        canvas.style.width = window.innerWidth;

        gl.viewport(0,0, window.innerWidth, window.innerHeight);
      }

      let elapsedTime = currentTime - intitialTime;
      client.update(elapsedTime, window.innerHeight, window.innerWidth);
      client.render();
    }
  }

  render();
});