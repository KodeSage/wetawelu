import { useCallback, useEffect, useState } from "react";
import Particles, { initParticlesEngine } from "@tsparticles/react";
import { loadSlim } from "@tsparticles/slim";
import particlesConfig from "./particles-config";

const ParticlesBackground = () => {
  const [init, setInit] = useState(false);
  useEffect(() => {
    initParticlesEngine(async (engine) => {
      await loadSlim(engine);
    }).then(() => {
      setInit(true);
    });
  }, []);

  const particlesLoaded = useCallback((container) => {
    console.log("Particles loaded", container);
  }, []);

  return (
    <div id="particles-background">
      {/* {init && (
     
      )} */}
      <Particles
        id="tsparticles"
        particlesLoaded="particlesLoaded"
        // loaded={particlesLoaded}
        // options={particlesConfig}
        height="100vh"
        width="100vw"
      />
    </div>
  );
};

export default ParticlesBackground;
