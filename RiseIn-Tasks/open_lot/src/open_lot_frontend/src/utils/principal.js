/**
 * Safely convert a Principal object to string
 * @param {*} principal - Principal object or string
 * @returns {string} - Principal as string
 */
export const principalToString = (principal) => {
  if (!principal) return '';
  
  if (typeof principal === 'string') {
    return principal;
  }
  
  if (typeof principal === 'object' && principal.toString) {
    return principal.toString();
  }
  
  if (typeof principal === 'object' && principal.toText) {
    return principal.toText();
  }
  
  return String(principal);
};

/**
 * Format a principal for display (truncated)
 * @param {*} principal - Principal object or string
 * @param {number} startChars - Number of characters to show at start
 * @param {number} endChars - Number of characters to show at end
 * @returns {string} - Formatted principal string
 */
export const formatPrincipal = (principal, startChars = 8, endChars = 4) => {
  const principalStr = principalToString(principal);
  if (!principalStr) return '';
  
  if (principalStr.length <= startChars + endChars + 3) {
    return principalStr;
  }
  
  return `${principalStr.slice(0, startChars)}...${principalStr.slice(-endChars)}`;
};

/**
 * Compare two principals for equality
 * @param {*} principal1 - First principal
 * @param {*} principal2 - Second principal
 * @returns {boolean} - Whether they are equal
 */
export const principalsEqual = (principal1, principal2) => {
  return principalToString(principal1) === principalToString(principal2);
}; 