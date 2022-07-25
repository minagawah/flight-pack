import './main.css';
import('../pkg/index.js').catch(console.error);
import { TARGET_AIRPORTS } from './airports';
import {
  load as load_googlemap,
  create_get_bounds,
  create_get_center,
} from './googlemap';

const FITBOUNDS_PADDING_PIXEL = 0;

(async () => {
  try {
    const wasm = await import('../pkg/index.js');
    const get_center = create_get_center(wasm);

    const el = {
      googlemap: document.querySelector('#googlemap'),
      canvas: document.querySelector('#flight'),
    };

    if (!el.googlemap) throw new Error('No element for Google map');
    if (!el.canvas) throw new Error('No element for flight info');

    const airports = TARGET_AIRPORTS;

    // When initializing Google map, it requires the center
    // from all the coordinates in TARGET_AIRPORTS.
    // WASM app provides `find_geo_center` (for which
    // we have a JS wrapper, called `get_center`)
    // to let you find the center.
    const coords = airports.map(({ coord }) => coord);

    let [google, map] = await load_googlemap({
      apiKey: process.env.GOOGLE_API_KEY,
      el: el.googlemap,
      options: { center: get_center(coords) },
    });

    const get_bounds = create_get_bounds(google);

    // The map should be ready.
    // We will ask Google to re-render the map
    // for all the airports to fit in the map.
    let bounds = get_bounds(coords);
    let center = bounds.getCenter();

    map.fitBounds(bounds, FITBOUNDS_PADDING_PIXEL);
    map.setCenter(center);

    let app;
    let actual_coords;

    // Listen to "zoom_changed" event.
    // Once the map is re-rendered, triggers
    // "zoom_changed" event, and that is
    // when we want to initialize the WASM app.
    map.addListener('zoom_changed', async () => {
      console.log('[index] (Event) "zoom_changed"');

      if (!!app || !!actual_coords) return;
      console.log('[index] Instantiating \'App\'');

      app = new wasm.App(el.canvas);

      // Ask the WASM app to fetch arrival/departure
      // information. Once the data is fetched,
      // we want to once again update the map for all
      // the coordinates to fit in the map.

      const response = await app.prepare(airports);
      actual_coords = JSON.parse(response);

      bounds = get_bounds(actual_coords);
      center = get_center(actual_coords);

      map.fitBounds(bounds, FITBOUNDS_PADDING_PIXEL);
      map.setCenter(center);

      app.start(); // Start animation loop
    });

    // Listen to "bounds_changed". Whenever the bounds
    // change, we will tell the WASM up to update.
    map.addListener('bounds_changed', () => {
      console.log('[index] (Event) "bounds_changed"');
      if (!!app && !!actual_coords) {
        console.log('[index] Running \'update()\'');
        app.update(map.getBounds().toJSON());
      }
    });
  } catch (err) {
    console.error(err);
  }
})();
