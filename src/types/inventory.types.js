/**
 * @typedef {Object} Suit
 * @property {string} ItemType - The type of the suit.
 * @property {Array<Object>} Configs - The configurations for the suit.
 * @property {number} UpgradeVer - The upgrade version of the suit.
 * @property {number} XP - The experience points of the suit.
 * @property {Date} [InfestationDate] - The date of infestation for the suit.
 * @property {number} [Features] - The features of the suit.
 * @property {Array<{Slot: number; Value: string}>} [Polarity] - Array of polarities for the suit.
 * @property {number} [Polarized] - Indicates if the suit is polarized.
 * @property {number} [ModSlotPurchases] - The number of mod slot purchases for the suit.
 * @property {{$oid:string}} [_id] - The unique object ID of the suit.
 */

/**
 * @typedef {Object} Ability
 * @property {string} uniqueName - The unique name of the ability.
 * @property {string} name - The name of the ability.
 * @property {string} description - The description of the ability.
 * @property {string} imageName - The image name associated with the ability.
 */

/**
 * @typedef {Object} Component
 * @property {string} uniqueName - The unique name of the component.
 * @property {string} name - The name of the component.
 * @property {string} description - The description of the component.
 * @property {number} itemCount - The number of items required for the component.
 * @property {string} imageName - The image name associated with the component.
 * @property {boolean} tradable - Indicates if the component is tradable.
 * @property {Array<Object>} drops - An array of objects representing drop chances and locations.
 * @property {boolean} masterable - Indicates if the component is masterable.
 * @property {string} [type] - The type of the component (optional).
 */

/**
 * @typedef {Object} SuitInfo
 * @property {Array<Ability>} abilities - An array of abilities associated with the suit.
 * @property {number} armor - The armor value of the suit.
 * @property {string} [aura] - The aura of the suit.
 * @property {number} [buildPrice] - The build price of the suit.
 * @property {number} [buildQuantity] - The build quantity of the suit.
 * @property {number} [buildTime] - The build time of the suit.
 * @property {string} category - The category of the suit.
 * @property {number} [color] - The color value of the suit.
 * @property {Array<Component>} [components] - An array of components required to build the suit.
 * @property {boolean} [conclave] - Indicates if the suit is usable in conclave.
 * @property {boolean} [consumeOnBuild] - Indicates if the suit is consumed on build.
 * @property {string} description - The description of the suit.
 * @property {number} health - The health value of the suit.
 * @property {string} imageName - The image name associated with the suit.
 * @property {boolean} isPrime - Indicates if the suit is a prime variant.
 * @property {number} [marketCost] - The market cost of the suit.
 * @property {boolean} masterable - Indicates if the suit is masterable.
 * @property {number} masteryReq - The mastery rank required to use the suit.
 * @property {string} name - The name of the suit.
 * @property {string} [passiveDescription] - The passive description of the suit.
 * @property {Array<string>} [polarities] - An array of polarities associated with the suit.
 * @property {number} power - The power value of the suit.
 * @property {string} productCategory - The product category of the suit.
 * @property {string} [releaseDate] - The release date of the suit.
 * @property {string} [sex] - The gender of the suit.
 * @property {number} shield - The shield value of the suit.
 * @property {number} [skipBuildTimePrice] - The price to skip build time for the suit.
 * @property {number} [sprint] - The sprint value of the suit.
 * @property {number} sprintSpeed - The sprint speed of the suit.
 * @property {number} stamina - The stamina value of the suit.
 * @property {boolean} tradable - Indicates if the suit is tradable.
 * @property {string} type - The type of the suit.
 * @property {string} uniqueName - The unique name of the suit.
 * @property {string} [wikiaThumbnail] - The URL of the suit's Wikia thumbnail.
 * @property {string} [wikiaUrl] - The URL of the suit's Wikia page.
 */

/**
 * @typedef {Object} SuitData
 * @property {string} ItemType - The type of the suit.
 * @property {Array<Object>} Configs - The configurations for the suit.
 * @property {number} UpgradeVer - The upgrade version of the suit.
 * @property {number} XP - The experience points of the suit.
 * @property {Date} [InfestationDate] - The date of infestation for the suit.
 * @property {number} [Features] - The features of the suit.
 * @property {Array<{Slot: number; Value: string}>} [Polarity] - Array of polarities for the suit.
 * @property {number} [Polarized] - Indicates if the suit is polarized.
 * @property {number} [ModSlotPurchases] - The number of mod slot purchases for the suit.
 * @property {{$oid:string}} [_id] - The unique object ID of the suit.
 * @property {SuitInfo} info - Information about the suit (from warframe-items).
 * @property {boolean} [_selected] - editor property.
 */

export default undefined;
