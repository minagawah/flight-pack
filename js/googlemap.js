import { Loader } from '@googlemaps/js-api-loader';

const DEFAULT_ZOOM_LEVEL = 9;

export const load = async ({ apiKey, el, options }) => {
  const google = await new Loader({ apiKey, version: 'weekly' }).load();
  const new_options = {
    zoom: DEFAULT_ZOOM_LEVEL,
    styles: GOOGLE_MAP_NIGHT_MODE_STYLES,
    disableDefaultUI: true,
    ...options,
  };
  const map = new google.maps.Map(el, new_options);

  return [google, map];
};

export const create_get_bounds =
  google =>
  (coords = {}) => {
    const bounds = new google.maps.LatLngBounds();
    coords.forEach(coord => {
      const { lat, lng } = coord;
      bounds.extend(new google.maps.LatLng(lat, lng));
    });
    return bounds;
  };

export const create_get_center = wasm => coords => {
  const [lat, lng] = wasm.find_geo_center(coords);
  return { lat, lng };
};

/** This is not in use. Instead, use: `wasm.find_geo_center` */
// export const calc_geo_center = (google, coords = {}) => {
//   const bounds = new google.maps.LatLngBounds();
//   coords.forEach(coord => {
//     const { lat, lng } = coord;
//     bounds.extend(new google.maps.LatLng(lat, lng));
//   });
//   return bounds.getCenter();
// };

export const GOOGLE_MAP_NIGHT_MODE_STYLES = [
  { elementType: 'geometry', stylers: [{ color: '#242f3e' }] },
  { elementType: 'labels.text.stroke', stylers: [{ color: '#242f3e' }] },
  { elementType: 'labels.text.fill', stylers: [{ color: '#746855' }] },
  {
    featureType: 'administrative.locality',
    elementType: 'labels.text.fill',
    stylers: [{ color: '#d59563' }],
  },
  {
    featureType: 'poi',
    elementType: 'labels.text.fill',
    stylers: [{ color: '#d59563' }],
  },
  {
    featureType: 'poi.park',
    elementType: 'geometry',
    stylers: [{ color: '#263c3f' }],
  },
  {
    featureType: 'poi.park',
    elementType: 'labels.text.fill',
    stylers: [{ color: '#6b9a76' }],
  },
  {
    featureType: 'road',
    elementType: 'geometry',
    stylers: [{ color: '#38414e' }],
  },
  {
    featureType: 'road',
    elementType: 'geometry.stroke',
    stylers: [{ color: '#212a37' }],
  },
  {
    featureType: 'road',
    elementType: 'labels.text.fill',
    stylers: [{ color: '#9ca5b3' }],
  },
  {
    featureType: 'road.highway',
    elementType: 'geometry',
    stylers: [{ color: '#746855' }],
  },
  {
    featureType: 'road.highway',
    elementType: 'geometry.stroke',
    stylers: [{ color: '#1f2835' }],
  },
  {
    featureType: 'road.highway',
    elementType: 'labels.text.fill',
    stylers: [{ color: '#f3d19c' }],
  },
  {
    featureType: 'transit',
    elementType: 'geometry',
    stylers: [{ color: '#2f3948' }],
  },
  {
    featureType: 'transit.station',
    elementType: 'labels.text.fill',
    stylers: [{ color: '#d59563' }],
  },
  {
    featureType: 'water',
    elementType: 'geometry',
    stylers: [{ color: '#17263c' }],
  },
  {
    featureType: 'water',
    elementType: 'labels.text.fill',
    stylers: [{ color: '#515c6d' }],
  },
  {
    featureType: 'water',
    elementType: 'labels.text.stroke',
    stylers: [{ color: '#17263c' }],
  },
];
